//! This library is designed to simplify the communication between a server agent, and a
//! central api server.
//! # Feature Flags
//! - `server`: Register what message types may be sent or recieved as a server.
//! - `client`: Register what message types may be sent or received as a client.
//!
//! Note that these features are mutually exclusive, attempting to use both of them will cause a compile error.
//!
//! The main difference between them is what can be sent and recieved down each socket. This helps to create limitations around what
//! each side of the connection must match against - all checked at compile time of course.
//!
//! # Examples
//! ```rust
//! #[cfg(feature = "server")]
//! async fn example() {
//!     use ws_com_framework::message::Message;
//!     use ws_com_framework::Sender;
//!     use ws_com_framework::Receiver;
//!
//!     let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
//!
//!     //Create a new sender over the sending stream of the websocket.
//!     let mut s = Sender::new(tx);
//!
//!     let message = "Hello, World!".to_owned();
//!
//!     //Same syntax, except message is now of our custom type, in this way we can limit what can be
//!     //sent down the websockets - which should help to reduce errors.
//!     s.send(message).await.unwrap();
//!
//!     //Close the websocket
//!     s.close().await;
//!
//!     let mut r = Receiver::new(rx); //Create a new reciever, which wraps over the sink of the websocket.
//!     while let Some(v) = r.next().await {
//!         //Very similar syntax to current solution
//!         //except that v is a custom type which we can then
//!         //easily match over
//!         assert_eq!(Message::Message("Hello, World!".into()), v.unwrap());
//!     }
//! }
//! ```

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "server")]
mod server;
mod traits;
pub mod error;
pub mod message;

pub use traits::{ RxStream, TxStream };


//Re-export relevant types
pub use error::Error;
pub use message::{FileId, PublicId, Passcode, Message};

//TODO implement feature flags for different traits. I.e. websockets, tungesnite, websockets, etc.

/// A wrapper over a websocket, is able to asynchronously send messages down the websocket.
#[derive(Clone, Copy, Debug)]
pub struct Sender<T>
where
    T: TxStream,
{
    tx: T,
}

impl<T> Sender<T>
where
    T: TxStream,
{
    /// Create a new sender, must be wrapped over a type which implements as TxStream to be valid.
    pub fn new(tx: T) -> Self {
        Self { tx }
    }

    /// Send a message down the pipeline to the reciever for this websocket.
    pub async fn send(&mut self, m: Message) -> Result<(), Error> {
        self.tx.__transmit(m).await
    }

    /// Close the sending side of this websocket connection.
    pub async fn close(self) -> Result<(), Error> {
        self.tx.__close()
    }

    ///Acquire the underlying tx stream, this consumes the sender wrapper.
    pub fn underlying(self) -> T {
        self.tx
    }
}

/// A wrapper over the receiving end of a websocket, will asychronously receive messages.
#[derive(Clone, Copy, Debug)]
pub struct Receiver<R>
where
    R: RxStream,
{
    rx: R,
}

impl<R> Receiver<R>
where
    R: RxStream,
{
    /// Create a new reciever, must be wrapped over a type which implements RxStream.
    pub fn new(rx: R) -> Self {
        Self { rx }
    }

    /// Asynchronously request the next message from the websocket
    /// Returns None in the event the websocket has closed and no other messages are coming
    /// Otherwise returns a result containing the value, for most implementation of the websocket
    /// the result is infalliable, however some types this may be wrapped over are not infallible.
    pub async fn next(&mut self) -> Option<Result<Message, Error>> {
        self.rx.__collect().await
    }

    ///Acquire the underlying rx stream, this consumes the reciever wrapper.
    pub fn underlying(self) -> R {
        self.rx
    }
}

//TODO lock certain functions in the general libraries (error + message) behind feature flags.
