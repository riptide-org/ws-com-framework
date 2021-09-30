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

// Disallow use of both server and client at the same time, this is because there are conflicting implemenations if
// they are both used. At some future date we could be more specific with our feature flags to make this not a problem
// but this is an acceptable bandaid solution.
#[cfg(all(feature = "client", feature = "server"))]
compile_error!(
    "features `ws-com-framework/client` and `ws-com-framework/server` are mutually exclusive"
);

#[cfg(feature = "client")]
mod client;
pub mod error;
pub mod message;
#[cfg(feature = "server")]
mod server;
mod traits;

//Re-export relevant types
pub use crate::error::{Error, ErrorLevel, WrappedError};
pub use crate::message::{AuthKey, File, Message, Request};

pub use traits::{RxStream, Sendable, TxStream};

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
    pub async fn send<E>(&mut self, m: E) -> Result<(), Error>
    where
        E: Into<Message> + Sendable,
    {
        self.tx.__transmit(m.into()).await
    }

    /// Close the sending side of this websocket connection.
    #[allow(unused_must_use)]
    pub async fn close(self) {
        //We do not need to check this return type, it wouldn't give us any useful information for doing so.
        self.tx.__close();
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

#[cfg(test)]
mod macros {
    use macros::IntoImpl;
    #[test]
    fn test_macro() {
        #[derive(IntoImpl, Debug, Eq, PartialEq)]
        #[allow(non_camel_case_types)]
        enum Test {
            String(String),
            i32(i32),
        }

        //Testing that the macro is working correctly
        let a: Test = String::from("Hello, world-1!").into();
        let b: Test = (25 as i32).into();

        assert_eq!(a, Test::String("Hello, world-1!".to_owned()));
        assert_eq!(b, Test::i32(25));
    }

    #[test]
    fn test_macro_extended() {
        mod further_structs {
            #[derive(Debug, Eq, PartialEq)]
            pub struct Hello {}
        }

        #[derive(IntoImpl, Debug, Eq, PartialEq)]
        enum Test {
            Other(i32),
            FurtherTest(further_structs::Hello),
        }

        let a: Test = (5 as i32).into();
        let b: Test = further_structs::Hello {}.into();

        assert_eq!(a, Test::Other(5));
        assert_eq!(b, Test::FurtherTest(further_structs::Hello {}))
    }

    #[test]
    fn test_macro_exclusion() {
        #[derive(IntoImpl, Debug, Eq, PartialEq)]
        enum Test {
            Hello(String),
            #[exclude]
            World(String),
            Other(i32),
        }

        let a: Test = "Hello".to_owned().into();
        let b: Test = "World".to_owned().into();
        let c: Test = (5 as i32).into();

        assert_eq!(a, Test::Hello("Hello".to_owned()));
        assert_eq!(b, Test::Hello("World".to_owned()));
        assert_ne!(a, Test::World("Hello".to_owned()));
        assert_ne!(b, Test::World("World".to_owned()));
        assert_eq!(c, Test::Other(5));
    }

    #[test]
    fn test_no_params() {
        #[allow(dead_code)]
        #[derive(IntoImpl, Debug, Eq, PartialEq)]
        enum Test {
            Hello(String),
            World,
        }

        let a: Test = "Hello".to_owned().into();

        assert_eq!(a, Test::Hello("Hello".into()));
    }
}
