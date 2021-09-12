use std::fmt::{Display, Formatter};

/// Represents an error when working with a websocket message. Whether serializing, deserializing, or parsing.
#[derive(Debug)]
pub enum WebsocketMessageError {
    ///Attempted to request a file with a uuid version != 4
    InvalidUuidVersion,
}

impl Display for WebsocketMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //TODO
        f.write_str("A websocket message error occured")
    }
}