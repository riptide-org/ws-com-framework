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
    Request(FileRequest)
}

impl TryFrom<Message> for WebsocketMessage {
    type Error = WebsocketMessageError;
    fn try_from(s: Message) -> Result<Self, WebsocketMessageError> {
        //TODO
        Ok(WebsocketMessage::Error("Not yet implemented".to_owned()))
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
}

impl File {
    fn new(name: String, size: usize, ext: String, user: String, crt: DateTime<Utc>, exp: DateTime<Utc>) -> File {
        File { name, size, ext, user, crt, exp }
    }
}

/// A request for the metadata for a file.
#[derive(Debug, Serialize, Deserialize)]
pub struct FileRequest {
    id: uuid::Uuid,
}

impl FileRequest {
    fn new(id: uuid::Uuid) -> Result<FileRequest, WebsocketMessageError> {
        //We want to recquire uuid's be generated with v4
        match id.get_version() {
            Some(uuid::Version::Random) => (),
            _ => return Err(WebsocketMessageError::InvalidUuidVersion)
        }
        Ok(FileRequest { id })
    }
}