//! Implementation for warp websockets

use crate::error::Error;
use crate::message::Message;
use crate::traits::{RxStream, TxStream};
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};

#[async_trait]
impl TxStream for futures::stream::SplitSink<warp::ws::WebSocket, warp::ws::Message> {
    async fn __transmit(&mut self, m: Message) -> Result<(), Error> {
        let msg: warp::ws::Message = TryFrom::try_from(m)?;

        self.send(msg)
            .await
            .map_err(|e| Error::SendFailure(e.to_string()))
    }
    #[allow(unused_must_use)]
    async fn __close(mut self) -> Result<(), Error> {
        self.close().await.map_err(|e| Error::CloseFailure(e.to_string()))
    }
}

#[async_trait]
impl RxStream for futures::stream::SplitStream<warp::ws::WebSocket> {
    async fn __collect(&mut self) -> Option<Result<Message, Error>> {
        match self.next().await {
            Some(Ok(f)) => Some(f.try_into()),
            Some(Err(e)) => Some(Err(Error::ReceiveFailure(e.to_string()))),
            None => None,
        }
    }
}

impl TryFrom<Message> for warp::ws::Message {
    type Error = Error;
    fn try_from(value: Message) -> Result<Self, Self::Error> {
        Ok(warp::ws::Message::binary(value.into_bytes()?))
    }
}

impl TryFrom<warp::ws::Message> for Message {
    type Error = Error;
    fn try_from(value: warp::ws::Message) -> Result<Self, Error> {
        if value.is_binary() {
            return Ok(Self::from_bytes(value.as_bytes())?)
        }
        if value.is_close() {
            return Ok(Self::Close)
        }
        todo!(); //TODO
    }
}

// impl std::convert::From<Message> for warp::ws::Message {
//     fn from(s: Message) -> warp::ws::Message {
//         let b = bincode::serialize(&s).expect("Serialisation of message failed!"); //Saftey: Static type, tested
//         warp::ws::Message::binary(b)
//     }
// }

// impl std::convert::Into<Message> for warp::ws::Message {
//     fn into(self) -> Message {
//         bincode::deserialize(self.as_bytes()).unwrap()
//     }
// }
