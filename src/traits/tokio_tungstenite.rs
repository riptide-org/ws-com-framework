//! Implementation for tokio-tungstenite

use async_trait::async_trait;
use crate::traits::{ RxStream, TxStream };
use crate::message::Message;
use crate::error::{ Error, ErrorLevel, WrappedError };
use futures::{ StreamExt, SinkExt };

#[cfg(feature = "wrapper-tungstenite")]
#[async_trait]
impl TxStream for futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Message> {
    async fn __transmit<T>(&mut self, m: T) -> Result<(), Error> 
    where
        T: Into<Message> + Send,
    {
        let m: Message = m.into();
        self.send(m.into()).await.map_err(|e| Error::Generic(WrappedError::new(ErrorLevel::High, e.to_string())))
    }
    #[allow(unused_must_use)]
    async fn __close(mut self) {
        self.close().await; //TODO refactor to return result
    }
}

#[cfg(feature = "wrapper-tungstenite")]
#[async_trait]
impl RxStream for futures_util::stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>> {
    async fn __collect<T>(&mut self) -> Option<Result<T, Error>>
    where
        T: From<Message> + Send,
    {
        if let Some(f) = self.next().await {
            //Convert type into intermediary
            let f: Result<Message, Error> = f
                .map(|m| m.into())
                .map_err(|e| Error::Generic(WrappedError::new(ErrorLevel::High, e.to_string())));
            
            //Convert our Message type into whatever type this client requires
            let f: Result<T, Error> = f.map(|m| m.into());
            return Some(f);
        }
        None
    }
}

#[cfg(feature = "wrapper-tungstenite")]
impl From<Message> for tokio_tungstenite::tungstenite::Message {
    fn from(s: Message) -> Self {
        let b = bincode::serialize(&s).expect("Serialisation of message failed!"); //Saftey: Static type, tested
        Self::Binary(b)
    }
}

#[cfg(feature = "wrapper-tungstenite")]
impl Into<Message> for tokio_tungstenite::tungstenite::Message {
    fn into(self) -> Message {
        return match self {
            tokio_tungstenite::tungstenite::Message::Binary(b) => {
                let b = &b;
                bincode::deserialize(b).unwrap()
            }
            tokio_tungstenite::tungstenite::Message::Close(e) => {
                let e = e.unwrap_or(tokio_tungstenite::tungstenite::protocol::CloseFrame {
                    reason: "Unknown Reason".into(),
                    code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Abnormal,
                });
                Message::Close(e.reason.to_string())
            }
            t => panic!("Type not implemented for websocket parsing: {:?}", t),
        };
    }
}