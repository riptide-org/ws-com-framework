//! Implementation for warp websockets

use crate::error::{ Error, ErrorLevel, WrappedError };
use crate::message::Message;
use crate::traits::{ RxStream, TxStream };
use async_trait::async_trait;
use futures::{ SinkExt, StreamExt };

#[cfg(feature = "wrapper-warp")]
#[async_trait]
impl TxStream for futures::stream::SplitSink<warp::ws::WebSocket, warp::ws::Message> {
    async fn __transmit<T>(&mut self, m: T) -> Result<(), Error>
    where
        T: Into<Message> + Send,
    {
        self.send(m.into().into())
            .await
            .map_err(|e| Error::Generic(WrappedError::new(ErrorLevel::High, e.to_string())))
    }
    #[allow(unused_must_use)]
    async fn __close(mut self) {
        self.close().await; //TODO refactor to return result
    }
}

#[cfg(feature = "wrapper-warp")]
#[async_trait]
impl RxStream for futures::stream::SplitStream<warp::ws::WebSocket> {
    async fn __collect<T>(&mut self) -> Option<Result<T, Error>>
    where
        T: From<Message> + Send,
    {
        let m: Result<Message, Error>;
        if let Some(t) = self.next().await {
            m = match t {
                Ok(f) => Ok::<Message, Error>(f.into()),
                Err(e) => Err(Error::Generic(WrappedError::new(
                    ErrorLevel::High,
                    e.to_string(),
                ))),
            };
            return Some(m.map(|f| f.into()));
        }
        None
    }
}

#[cfg(feature = "wrapper-warp")]
impl std::convert::From<Message> for warp::ws::Message {
    fn from(s: Message) -> warp::ws::Message {
        let b = bincode::serialize(&s).expect("Serialisation of message failed!"); //Saftey: Static type, tested
        warp::ws::Message::binary(b)
    }
}

#[cfg(feature = "wrapper-warp")]
impl std::convert::Into<Message> for warp::ws::Message {
    fn into(self) -> Message {
        bincode::deserialize(self.as_bytes()).unwrap()
    }
}