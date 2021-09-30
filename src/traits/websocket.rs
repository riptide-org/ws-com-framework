//! Implmenation for websockets split streams

use crate::error::{ Error, ErrorLevel, WrappedError };
use crate::message::Message;
use crate::traits::{ RxStream, TxStream };
use async_trait::async_trait;

#[cfg(feature="wrapper-websocket")]
#[async_trait]
impl TxStream for websocket::sender::Writer<std::net::TcpStream> {
    async fn __transmit<T>(&mut self, m: T) -> Result<(), Error>
    where
        T: Into<Message> + Send,
    {
        let m: Message = m.into();
        let m: websocket::OwnedMessage = m.into();
        self.send_message(&m)
            .map_err(|e| Error::Generic(WrappedError::new(ErrorLevel::High, e.to_string())))
    }

    #[allow(unused_must_use)]
    async fn __close(self) {
        self.shutdown_all();
    }
}

#[cfg(feature="wrapper-websocket")]
#[async_trait]
impl RxStream for websocket::receiver::Reader<std::net::TcpStream> {
    async fn __collect<T>(&mut self) -> Option<Result<T, Error>>
    where
        T: From<Message> + Send,
    {
        let m: Message = match self.recv_message() {
            Ok(f) => f.into(),
            Err(e) => {
                return match e {
                    websocket::result::WebSocketError::NoDataAvailable => None,
                    _ => Some(Err(Error::Generic(WrappedError::new(
                        ErrorLevel::High,
                        e.to_string(),
                    )))),
                };
            }
        };

        return Some(Ok(m.into()));
    }
}

#[cfg(feature="wrapper-websocket")]
impl From<Message> for websocket::OwnedMessage {
    fn from(s: Message) -> websocket::OwnedMessage {
        let b = bincode::serialize(&s).expect("Serialisation of message failed!"); //Saftey: Static type, tested
        Self::Binary(b)
    }
}

#[cfg(feature="wrapper-websocket")]
impl Into<Message> for websocket::OwnedMessage {
    fn into(self) -> Message {
        return match self {
            websocket::OwnedMessage::Binary(b) => {
                let b = &b;
                bincode::deserialize(b).unwrap()
            }
            websocket::OwnedMessage::Close(e) => {
                let e = e.unwrap_or(websocket::CloseData {
                    reason: "Unknown Reason".into(),
                    status_code: 400,
                });
                Message::Close(e.reason)
            }
            t => panic!("Type not implemented for websocket parsing: {:?}", t),
        };
    }
}