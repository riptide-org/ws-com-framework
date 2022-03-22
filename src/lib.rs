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


pub mod error;
pub mod message;

//Re-export relevant types
pub use crate::error::Error;