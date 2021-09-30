use crate::error::Error;
use crate::traits::{Receivable, Sendable};
use chrono::prelude::*;
use macros::IntoImpl;
use serde::{Deserialize, Serialize};

/// A message which can be sent between the server and client. Can hold
/// A variety of values and types, depending on which action needs to be
/// carried out.
#[derive(Debug, Serialize, Deserialize, IntoImpl, Eq, PartialEq, Clone)]
pub enum Message {
    Error(Error),
    MetadataResponse(Upload),
    Message(String),
    #[exclude]
    Close(String),
    AuthReq,
    #[exclude]
    AuthResponse(AuthKey),
    #[exclude]
    MetadataRequest(Request),
    #[exclude]
    UploadRequest(Request),
}

impl Sendable for Message {}
impl Receivable for Message {}

/// A metadata upload back to the server, should only be sent from agent -> server
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Upload {
    stream_id: String,
    payload: File,
}

impl Upload {
    /// Create new upload response
    pub fn new(stream_id: String, payload: File) -> Upload {
        Upload { stream_id, payload }
    }
    ///Get the stream id
    pub fn get_stream_id(&self) -> &str {
        &self.stream_id
    }
    ///Set stream id
    pub fn set_stream_id(&mut self, stream_id: String) {
        self.stream_id = stream_id;
    }
    ///Get the payload, this consumes the payload
    pub fn get_payload(self) -> File {
        self.payload
    }
}

/// Represents a file saved on the users system, this is the metadata of which sent to the server.
/// This should be sent in response to a FileRequest being recieved
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct File {
    id: uuid::Uuid,
    user: String,
    crt: DateTime<Utc>,
    exp: DateTime<Utc>,
    website: bool,
    wget: bool,
    name: String,
    size: usize,
    ext: String,
}

impl File {
    pub fn new(
        id: uuid::Uuid,
        crt: DateTime<Utc>,
        exp: DateTime<Utc>,
        user: String,
        website: bool,
        wget: bool,
        name: String,
        size: usize,
        ext: String,
    ) -> File {
        File {
            id,
            user,
            crt,
            exp,
            website,
            wget,
            name,
            size,
            ext,
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

/// A request from the server -> agent for the agent to upload some data
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Request {
    id: uuid::Uuid,
    url: String,
}

impl Request {
    pub fn new(id: uuid::Uuid, url: String) -> Self {
        Self { id, url }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

/// An authorisation response, should be sent only from client -> Server
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct AuthKey {
    pub key: Vec<u8>,
}
