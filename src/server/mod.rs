//! Communication framework to be used on the server-side of the websocket connection.

mod traits;
mod error;
mod message;
use traits::*;
use error::Error;
use message::Message;
use futures::SinkExt;
use async_trait::async_trait;

pub struct Sender<T> 
where T: TxStream 
{
    tx: T
}

impl<T> Sender<T> 
where T: TxStream
{
    pub fn new(tx: T) -> Self 
    {
        Self { tx }
    }

    pub async fn send<E>(&mut self, m: E) -> Result<(), Error>
    where E: Into<Message> + Sendable
    {
        self.tx.transmit(m.into()).await
    }

    #[allow(unused_must_use)]
    pub async fn close(self) {
        self.tx.close();
    }
}

pub struct Receiver<R> 
where R: RxStream {
    rx: R
}

impl<R> Receiver<R> 
where R: RxStream {
    pub fn new(rx: R) -> Self {
        Self { rx }
    }

    pub async fn next(&mut self) -> Option<Result<Message, Error>> 
    {
        self.rx.collect().await
    }
}

//////// Implementation for warp websockets ///////////

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

impl std::convert::From<Message> for warp::ws::Message {
    fn from(s: Message) -> warp::ws::Message {
        warp::ws::Message::text("test")
    }
}

//////// Implementation for tokios unbounded sender ///////////

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


//////// Tests ////////

#[cfg(test)]
mod tests {
    use crate::server::{Sender, Receiver}; 
    use crate::server::message::Message;

    #[tokio::test]
    ///Test that basic functionality works. Creates a simple unbounded channel and sends some messages down it.
    async fn basic_functionality() {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
    
        let mut s = Sender::new(tx); //Create a new sender over the sending stream of the websocket.
    
        let message = "Hello, World!".to_owned();
    
        //Same syntax, except message is now of our custom type, in this way we can limit what can be
        //sent down the websockets - which should help to reduce errors.
        s.send(message).await.unwrap();
    
        //Close the websocket 
        s.close().await;
    
        let mut r = Receiver::new(rx); //Create a new reciever, which wraps over the sink of the websocket.
        while let Some(v) = r.next().await {
            //Very similar syntax to current solution
            //except that v is a custom type which we can then
            //easily match over
            assert_eq!(Message::Message("Hello, World!".into()), v.unwrap());
        }
    }
}