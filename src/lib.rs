//! Ws-com-framework converts messages to and from binary for sending down web sockets.
//!
//! The `Message` type implements `TryFrom` and `TryInto` for `Vec<u8>`, and is designed to be
//! matched against for processing/responding to requests.
//!
//! # Example
//! ```rust
//! async fn example() {
//!     use ws_com_framework::message::Message;
//!
//!     let (mut tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
//!
//!     let message: Message = Message::AuthReq {
//!         public_id: 43
//!     };
//!     tx.send(message.try_into().unwrap()).unwrap();
//!
//!     while let Some(v) = rx.recv().await {
//!         let recv_message = Message::try_from(v).unwrap();
//!         assert_eq!(Message::AuthReq{public_id: 43}, recv_message);
//!     }
//! }
//! ```

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    deprecated
)]

pub mod error;
pub mod message;

//Re-export relevant types
pub use error::Error;
pub use message::{FileId, Message, Passcode, PublicId, UploadId};
