//! Contains traits used by the crate
use crate::error::Error;
use crate::message::Message;
use async_trait::async_trait;

// Individual configurations which can be loaded depending on what features are required.
#[cfg(feature = "wrapper-tokio")]
mod tokio;
#[cfg(feature = "wrapper-tungstenite")]
mod tokio_tungstenite;
#[cfg(feature = "wrapper-warp")]
mod warp;
#[cfg(feature = "wrapper-websocket")]
mod websocket;

//////// Traits ////////

/// A trait indicating this is a valid tx stream, and therefore will implement the required methods.
#[doc(hidden)]
#[async_trait]
pub trait TxStream {
    /// send a message doesn this stream
    async fn __transmit(&mut self, m: Message) -> Result<(), Error>;
    /// close this transmission stream
    async fn __close(self) -> Result<(), Error>;
}

/// A trait indicating this is a valid rx stream, and therefore will implement the required methods.
#[doc(hidden)]
#[async_trait]
pub trait RxStream {
    /// attempt to recieve a message from this stream
    async fn __collect(&mut self) -> Option<Result<Message, Error>>;
}
