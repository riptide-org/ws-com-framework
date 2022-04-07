//! Implmenation for websockets split streams

use crate::error::Error;
use crate::message::Message;
use crate::traits::{RxStream, TxStream};
use async_trait::async_trait;

#[async_trait]
impl TxStream for websocket::sender::Writer<std::net::TcpStream> {
    async fn __transmit(&mut self, m: Message) -> Result<(), Error> {
        let m: websocket::OwnedMessage = m.try_into()?;
        self.send_message(&m)
            .map_err(|e| Error::SendFailure(e.to_string()))
    }

    #[allow(unused_must_use)]
    async fn __close(self) -> Result<(), Error> {
        self.shutdown_all()
            .map_err(|e| Error::CloseFailure(e.to_string()))
    }
}

#[async_trait]
impl RxStream for websocket::receiver::Reader<std::net::TcpStream> {
    async fn __collect(&mut self) -> Option<Result<Message, Error>> {
        match self.recv_message() {
            Ok(f) => Some(f.try_into()),
            Err(websocket::result::WebSocketError::NoDataAvailable) => None,
            Err(e) => Some(Err(Error::ReceiveFailure(e.to_string()))),
        }
    }
}

impl TryFrom<Message> for websocket::OwnedMessage {
    type Error = Error;
    fn try_from(s: Message) -> Result<websocket::OwnedMessage, Error> {
        Ok(Self::Binary(s.into_bytes()?))
    }
}

impl TryFrom<websocket::OwnedMessage> for Message {
    type Error = Error;
    fn try_from(value: websocket::OwnedMessage) -> Result<Self, Error> {
        return match value {
            websocket::OwnedMessage::Binary(ref b) => Message::from_bytes(b),
            websocket::OwnedMessage::Close(_) => Ok(Message::Close), //XXX: Parse close reason?
            t => panic!("type not implemented for websocket parsing: {:?}", t),
        };
    }
}
