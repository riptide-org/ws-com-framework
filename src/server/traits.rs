//! Contains traits needed for the server side of this application

use crate::server::error::Error;
use crate::server::Message;
use futures::SinkExt;
use async_trait::async_trait;

//////// Traits ////////


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

//////// Trait Impls ////////
#[async_trait]
impl TxStream for futures::stream::SplitSink<warp::ws::WebSocket, warp::ws::Message> {
    async fn transmit<T>(&mut self, m: T) -> Result<(), Error>
    where T: Into<Message> + Send
    {
        //TODO Error handling
        self.send(m.into().into()).await.map_err(|_| Error::A)
    }
    async fn close(self) {
        self.close().await;
    }
}

#[async_trait]
impl TxStream for tokio::sync::mpsc::UnboundedSender<Message> {
    async fn transmit<T>(&mut self, m: T) -> Result<(), Error> 
    where T: Into<Message> + Send
    {
        //TODO Error handling
        self.send(m.into()).map_err(|_| Error::A)
    }
    async fn close(self) {
        self.close().await;
    }
}

#[async_trait]
impl RxStream for tokio::sync::mpsc::UnboundedReceiver<Message> {
    async fn collect<T>(&mut self) -> Option<Result<T, Error>> 
    where T: From<Message> + Send
    {
        if let Some(t) = self.recv().await {
            return Some(Ok(t.into()))
        }
        None
    }
}

impl std::convert::From<Message> for warp::ws::Message {
    fn from(s: Message) -> warp::ws::Message {
        warp::ws::Message::text("test")
    }
}