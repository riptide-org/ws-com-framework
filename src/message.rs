use crate::error::Error;
use crate::traits::{Receivable, Sendable};
use chrono::prelude::*;
use macros::IntoImpl;
use serde::{Deserialize, Serialize};

pub mod websocket_message {
    include!(concat!(env!("OUT_DIR"), "/events.rs"));
}

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
    pub stream_id: String,
    pub payload: File,
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
    pub id: [u8; 6],
    pub user: String,
    pub crt: DateTime<Utc>,
    pub exp: DateTime<Utc>,
    pub hash: [u8; 32],
    pub name: String,
    pub size: usize,
    pub ext: String,
}

/// A request from the server -> agent for the agent to upload some data
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Request {
    pub id: [u8; 6],
    pub url: String,
}

/// An authorisation response, should be sent only from client -> Server
/// Should contain 32 bytes of randomly generated hex data.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct AuthKey {
    pub key: [u8; 32],
}
