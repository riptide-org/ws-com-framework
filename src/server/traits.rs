//! Contains traits needed for the server

use crate::server::error::Error;
use crate::server::Message;
use async_trait::async_trait;

//////// Traits ////////

/// A message type which can be sent by the server.
pub trait Sendable {}

/// A message type which can be recieved by the server.
pub trait Receivable {}

/// A trait indicating this is a valid tx stream, and therefore will implement the required methods.
#[async_trait]
pub trait TxStream {
    async fn transmit<T>(&mut self, m: T) -> Result<(), Error>
    where T: Into<Message> + Send;
    async fn close(self);
}

/// A trait indicating this is a valid rx stream, and therefore will implement the required methods.
#[async_trait]
pub trait RxStream { 
    async fn collect<T>(&mut self) -> Option<Result<T, Error>>
    where T: From<Message> + Send;
}