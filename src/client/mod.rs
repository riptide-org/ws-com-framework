//! Communication framework to be used on the server-side of the websocket connection.
use crate::error::Error;
use crate::message::{File, FileRequest, FileUploadRequest};
use crate::traits::{Receivable, Sendable};

//TODO macro-ise these, so they can be quickly derived rather than manually doing this.
impl Receivable for Error {}
impl Sendable for Error {}
impl Receivable for FileRequest {}
impl Sendable for File {}
impl Receivable for FileUploadRequest {}
impl Receivable for String {}
impl Sendable for String {}

//////// Tests ////////
