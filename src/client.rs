//! Communication framework to be used on the server-side of the websocket connection.
use crate::error::Error;
use crate::message::{File, Request, Upload};
use crate::traits::{Receivable, Sendable};

//TODO macro-ise these, so they can be quickly derived rather than manually doing this.
impl Receivable for Error {}
impl Sendable for Error {}
impl Receivable for Request {}
impl Receivable for String {}
impl Sendable for String {}
impl Sendable for File {}
impl Sendable for Upload {}


//////// Tests ////////
