//! Implementation for tokio-tungstenite

use crate::error::Error;
use crate::message::Message;
use crate::traits::{RxStream, TxStream};
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};

#[async_trait]
impl TxStream
    for futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        tokio_tungstenite::tungstenite::Message,
    >
{
    async fn __transmit(&mut self, m: Message) -> Result<(), Error> {
        self.send(m.try_into()?)
            .await
            .map_err(|e| Error::SendFailure(Box::new(e)))
    }
    #[allow(unused_must_use)]
    async fn __close(mut self) {
        self.close().await; //TODO refactor to return result
    }
}

#[async_trait]
impl RxStream
    for futures_util::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    >
{
    async fn __collect(&mut self) -> Option<Result<Message, Error>> {
        if let Some(f) = self.next().await {
            //Convert type into intermediary
            return Some(match f {
                Ok(msg) => TryFrom::try_from(msg),
                Err(e) => Err(Error::ReceiveFailure(Box::new(e))),
            })
        }
        None
    }
}

impl TryFrom<Message> for tokio_tungstenite::tungstenite::Message {
    type Error = Error;
    fn try_from(s: Message) -> Result<Self, Self::Error> {
        Ok(Self::Binary(s.into_bytes()?))
    }
}

impl TryFrom<tokio_tungstenite::tungstenite::Message> for Message {
    type Error = Error;
    fn try_from(s: tokio_tungstenite::tungstenite::Message) -> Result<Self, Error> {
        match s {
            tokio_tungstenite::tungstenite::Message::Text(_) => todo!(),
            tokio_tungstenite::tungstenite::Message::Binary(_) => todo!(),
            tokio_tungstenite::tungstenite::Message::Ping(_) => todo!(),
            tokio_tungstenite::tungstenite::Message::Pong(_) => todo!(),
            tokio_tungstenite::tungstenite::Message::Close(_) => todo!(),
            tokio_tungstenite::tungstenite::Message::Frame(_) => todo!(),
        }
    }
}

// impl Into<Message> for tokio_tungstenite::tungstenite::Message {
//     fn into(self) -> Message {
//         return match self {
//             tokio_tungstenite::tungstenite::Message::Binary(b) => {
//                 let b = &b;
//                 bincode::deserialize(b).unwrap()
//             }
//             tokio_tungstenite::tungstenite::Message::Close(e) => {
//                 let e = e.unwrap_or(tokio_tungstenite::tungstenite::protocol::CloseFrame {
//                     reason: "Unknown Reason".into(),
//                     code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Abnormal,
//                 });
//                 Message::Close(e.reason.to_string())
//             }
//             t => panic!("Type not implemented for websocket parsing: {:?}", t),
//         };
//     }
// }
