//! Implementation for tokios unbounded sender

use crate::error::{ Error, ErrorLevel, WrappedError };
use crate::message::Message;
use crate::traits::{ RxStream, TxStream };
use async_trait::async_trait;

#[cfg(feature = "wrapper-tokio")]
#[async_trait]
impl TxStream for tokio::sync::mpsc::UnboundedSender<Message> {
    async fn __transmit<T>(&mut self, m: T) -> Result<(), Error>
    where
        T: Into<Message> + Send,
    {
        self.send(m.into())
            .map_err(|e| Error::Generic(WrappedError::new(ErrorLevel::High, e.to_string())))
    }
    async fn __close(self) {
        //Doesn't exist! This should merely be dropped out of scope to close.
    }
}

#[cfg(feature = "wrapper-tokio")]
#[async_trait]
impl RxStream for tokio::sync::mpsc::UnboundedReceiver<Message> {
    async fn __collect<T>(&mut self) -> Option<Result<T, Error>>
    where
        T: From<Message> + Send,
    {
        if let Some(t) = self.recv().await {
            return Some(Ok(t.into()));
        }
        None
    }
}