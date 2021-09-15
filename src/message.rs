use crate::error::Error;
use crate::traits::{Receivable, Sendable};
use chrono::prelude::*;
use macros::IntoImpl;
use serde::{Deserialize, Serialize};

/// A message which can be sent between the server and client. Can hold
/// A variety of values and types, depending on which action needs to be
/// carried out.
#[derive(Debug, Serialize, Deserialize, IntoImpl, Eq, PartialEq)]
pub enum Message {
    Error(Error),
    Metadata(FileRequest),
    File(File),
    Upload(FileUploadRequest),
    Message(String),
    #[exclude]
    Close(String),
}

impl Sendable for Message {}
impl Receivable for Message {}

/// A request for the metadata for a file.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct FileRequest {
    id: uuid::Uuid,
    stream_id: usize,
}

impl FileRequest {
    /// Create a new request for file metadata
    pub fn new(id: uuid::Uuid, stream_id: usize) -> Result<FileRequest, Error> {
        //We want to recquire uuid's be generated with v4
        match id.get_version() {
            Some(uuid::Version::Random) => (),
            _ => return Err(Error::InvalidUuidVersion),
        }
        Ok(FileRequest { id, stream_id })
    }

    /// Get the id of the file that has been requested
    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    /// Get the id of the stream this information is going to be sent to
    pub fn stream_id(&self) -> usize {
        self.stream_id
    }
}

/// Represents a file saved on the users system, this is the metadata of which sent to the server.
/// This should be sent in response to a FileRequest being recieved
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct File {
    id: uuid::Uuid,
    name: String,
    size: usize,
    ext: String,
    user: String,
    crt: DateTime<Utc>,
    exp: DateTime<Utc>,
    stream_id: usize,
}

impl File {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        size: usize,
        ext: String,
        user: String,
        crt: DateTime<Utc>,
        exp: DateTime<Utc>,
        stream_id: usize,
    ) -> File {
        File {
            id,
            name,
            size,
            ext,
            user,
            crt,
            exp,
            stream_id,
        }
    }

    pub fn stream_id(&self) -> usize {
        self.stream_id
    }
}

/// A request from the server -> agent, the agent should upload the file with the specified id
/// to the url if it has it. Otherwise, it should respond with an error.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct FileUploadRequest {
    id: uuid::Uuid,
    url: String,
}

impl FileUploadRequest {
    pub fn new(id: uuid::Uuid, url: String) -> Self {
        FileUploadRequest { id, url }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }
}
