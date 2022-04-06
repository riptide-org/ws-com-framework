//! Implementation for tokios unbounded sender

use crate::error::Error;
use crate::message::Message;
use crate::traits::{RxStream, TxStream};
use async_trait::async_trait;

#[async_trait]
impl TxStream for tokio::sync::mpsc::UnboundedSender<Message> {
    async fn __transmit(&mut self, m: Message) -> Result<(), Error> {
        self.send(m)
            .map_err(|e| Error::SendFailure(Box::new(e)))
    }

    async fn __close(self) {
        //Doesn't exist! This should merely be dropped out of scope to close.
        drop(self);
    }
}

#[async_trait]
impl RxStream for tokio::sync::mpsc::UnboundedReceiver<Message> {
    async fn __collect(&mut self) -> Option<Result<Message, Error>> {
        match self.recv().await {
            Some(t) => Some(Ok(t.into())),
            None => None,
        }
    }
}
