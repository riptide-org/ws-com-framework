use serde::{Deserialize, Serialize};
use crate::server::error::Error;
use crate::server::traits::{Sendable, Receivable};
use macros::IntoImpl;

/// A message which can be sent between the server and client. Can hold
/// A variety of values and types, depending on which action needs to be
/// carried out.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, IntoImpl)]
pub enum Message {
    /// An error has occured, this is bidirectional between server <--> agent.
    Error(Error),
    /// Contains a request for metadata on a file, only server -> agent.
    Metadata(FileRequest),
    /// Contains metadata about a file, only agent -> server.
    File(File),
    /// A request from server -> agent to upload a file
    Upload(FileUploadRequest),
    /// A simple message, largely used in testing.
    Message(String),
}

impl Sendable for Error {}
impl Receivable for Error {}
impl Sendable for FileRequest {}
impl Receivable for File {}
impl Sendable for FileUploadRequest {}
impl Sendable for String {}
impl Receivable for String {}

///TODO make these actual types
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileRequest {}
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct File {}
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileUploadRequest {}