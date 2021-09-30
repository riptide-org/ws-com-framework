//! Communication framework to be used on the server-side of the websocket connection.
use crate::error::Error;
use crate::message::{File, Request};
use crate::traits::{Receivable, Sendable};

//TODO macro-ise these, so they can be quickly derived rather than manually doing this.
impl Sendable for Error {}
impl Receivable for Error {}
impl Receivable for File {}
impl Sendable for Request {}
impl Sendable for String {}
impl Receivable for String {}

//////// Tests ////////

#[cfg(test)]
mod server {
    use crate::message::Message;
    use crate::{Receiver, Sender};

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
