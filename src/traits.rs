//! Contains traits used by the crate
use crate::error::{Error, ErrorLevel, WrappedError};
use crate::message::Message;
use async_trait::async_trait;
use futures::SinkExt;
use futures::StreamExt;

//////// Traits ////////

/// A message type which can be sent.
pub trait Sendable {}

/// A message type which can be recieved.
pub trait Receivable {}

/// A trait indicating this is a valid tx stream, and therefore will implement the required methods.
#[async_trait]
pub trait TxStream {
    async fn transmit<T>(&mut self, m: T) -> Result<(), Error>
    where
        T: Into<Message> + Send;
    async fn close(self);
}

/// A trait indicating this is a valid rx stream, and therefore will implement the required methods.
#[async_trait]
pub trait RxStream {
    async fn collect<T>(&mut self) -> Option<Result<T, Error>>
    where
        T: From<Message> + Send;
}

//////// Implementation for warp websockets ///////////

#[async_trait]
impl TxStream for futures::stream::SplitSink<warp::ws::WebSocket, warp::ws::Message> {
    async fn transmit<T>(&mut self, m: T) -> Result<(), Error>
    where
        T: Into<Message> + Send,
    {
        self.send(m.into().into())
            .await
            .map_err(|e| Error::Generic(WrappedError::new(ErrorLevel::High, e.to_string())))
    }
    async fn close(self) {
        self.close().await;
    }
}

#[async_trait]
impl RxStream for futures::stream::SplitStream<warp::ws::WebSocket> {
    async fn collect<T>(&mut self) -> Option<Result<T, Error>>
    where T: From<Message> + Send,
    {
        let m: Result<Message, Error>;
        if let Some(t) = self.next().await {
            m = match t {
                Ok(f) => Ok::<Message, Error>(f.into()),
                Err(e) => Err(Error::Generic(WrappedError::new(ErrorLevel::High, e.to_string())))
            };
            return Some(m.map(|f| f.into()));
        }
        None
    }
}

impl std::convert::From<Message> for warp::ws::Message {
    fn from(s: Message) -> warp::ws::Message {
        let b = bincode::serialize(&s).expect("Serialisation of message failed!"); //Saftey: Static type, tested
        warp::ws::Message::binary(b)
    }
}

impl std::convert::Into<Message> for warp::ws::Message {
    fn into(self) -> Message {
        bincode::deserialize(self.as_bytes()).unwrap()
    }
}

//////// Implementation for tokios unbounded sender ///////////

#[async_trait]
impl TxStream for tokio::sync::mpsc::UnboundedSender<Message> {
    async fn transmit<T>(&mut self, m: T) -> Result<(), Error>
    where
        T: Into<Message> + Send,
    {
        self.send(m.into())
            .map_err(|e| Error::Generic(WrappedError::new(ErrorLevel::High, e.to_string())))
    }
    async fn close(self) {
        self.close().await;
    }
}

#[async_trait]
impl RxStream for tokio::sync::mpsc::UnboundedReceiver<Message> {
    async fn collect<T>(&mut self) -> Option<Result<T, Error>>
    where
        T: From<Message> + Send,
    {
        if let Some(t) = self.recv().await {
            return Some(Ok(t.into()));
        }
        None
    }
}
