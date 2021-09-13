//! Communication framework to be used on the server-side of the websocket connection.

mod traits;
mod error;
use traits::*;
use error::Error;

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

    pub async fn send(&mut self, m: Message) -> Result<(), Error> 
    {
        self.tx.transmit(m).await
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

#[derive(Debug, Eq, PartialEq)]
pub enum Message {
    A(String)
}

#[tokio::test]
///Test that basic functionality works. Creates a simple unbounded channel and sends some messages down it.
async fn basic_functionality() {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

    let mut s = Sender::new(tx); //Create a new sender over the sending stream of the websocket.

    let message = Message::A("Hello, World!".into());

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
        assert_eq!(Message::A("Hello, World!".into()), v.unwrap());
    }
}