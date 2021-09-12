//! Represents a websocket message, this will *only* be transmitted in binary form.
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use warp::ws::Message;
use std::fmt::{Display, Formatter};
use std::convert::TryFrom;
use crate::error::WebsocketMessageError;

/// Provided are parser methods to and from websocket binary messages.
#[derive(Serialize, Deserialize, Debug)]
pub enum WebsocketMessage {
    /// An error has occured, this is generally from server -> agent, not the other way around.
    /// These should be displayed to the user in an error format.
    Error(String),
    /// A message for the user of the app, usually from server -> agent. 
    /// Should be displayed to the user in a friendly format.
    Message(String),
    /// Contains file metadata, only agent -> server.
    File(File),
    /// Contains a request for file data, only server -> agent.
    Request(FileRequest),
    /// A request from server -> agent to upload a file to the url
    Upload(FileUploadRequest)
}

impl TryFrom<Message> for WebsocketMessage {
    type Error = WebsocketMessageError;
    fn try_from(s: Message) -> Result<Self, WebsocketMessageError> {
        if s.is_binary() {
            bincode::deserialize(s.as_bytes()).map_err(|_| WebsocketMessageError::FailedSerialization)
        } else {
            Err(WebsocketMessageError::FailedSerialization)
        }
    }
}

impl From<WebsocketMessage> for Message {
    fn from(s: WebsocketMessage) -> Message {
        match bincode::serialize(&s).map_err(|e| WebsocketMessage::Error(format!("Failed to serialize message: {}", e))) {
            Ok(f) => Message::binary(f),
            Err(e) => Message::binary(bincode::serialize(&WebsocketMessage::Error(e.to_string())).unwrap()),
        }
    }
}

impl Display for WebsocketMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //TODO
        f.write_str("Websocket Message (not yet implemented)")
    }
}


/// Represents a file saved on the users system, this is the metadata of which sent to the server.
#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    name: String,
    size: usize,
    ext: String,
    user: String,
    crt: DateTime<Utc>,
    exp: DateTime<Utc>,
    stream_id: usize,
}

impl File {
    pub fn new(name: String, size: usize, ext: String, user: String, crt: DateTime<Utc>, exp: DateTime<Utc>, stream_id: usize) -> File {
        File { name, size, ext, user, crt, exp, stream_id }
    }

    pub fn stream_id(&self) -> usize {
        self.stream_id
    }
}

/// A request for the metadata for a file.
#[derive(Debug, Serialize, Deserialize)]
pub struct FileRequest {
    id: uuid::Uuid,
    stream_id: usize,
}

impl FileRequest {
    pub fn new(id: uuid::Uuid, stream_id: usize) -> Result<FileRequest, WebsocketMessageError> {
        //We want to recquire uuid's be generated with v4
        match id.get_version() {
            Some(uuid::Version::Random) => (),
            _ => return Err(WebsocketMessageError::InvalidUuidVersion)
        }
        Ok(FileRequest { id, stream_id })
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn stream_id(&self) -> usize {
        self.stream_id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileUploadRequest {
    id: uuid::Uuid,
    url: String,
}

impl FileUploadRequest {
    pub fn new(id: uuid::Uuid, url: String) -> Self {
        FileUploadRequest{ id, url }
    }
}