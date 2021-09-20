//! Handles errors

use serde::{Deserialize, Serialize};

/// Represents an error when working with a websocket message. Whether serializing, deserializing, or parsing.
/// Also works as a wrapper for errors between the server and server agents.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum Error {
    /// Attempted to request a file with a uuid version != 4
    InvalidUuidVersion,
    /// File upload failed
    FailedFileUpload,
    /// The requested file doesn't exist
    FileDoesntExist,
    // Unable to serialize this file
    FailedSerialization,
    /// A wrapper over a generic error type
    Generic(WrappedError),
    /// Websocket not authenticated or disabled
    InvalidSession,
}

/// A generic error wrapper over the error types between different applications which may send messages.
/// Applications are expected to send and parse this error type as needed.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct WrappedError {
    level: ErrorLevel,
    message: String,
}

impl WrappedError {
    pub fn new<S: AsRef<str>>(level: ErrorLevel, message: S) -> WrappedError {
        let message = message.as_ref().to_owned();
        WrappedError { level, message }
    }
}

impl Default for WrappedError {
    fn default() -> WrappedError {
        WrappedError {
            level: ErrorLevel::Critical,
            message: "An unspecified error occured!".to_owned(),
        }
    }
}

/// Dictates the criticality of a sent/recieved error.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum ErrorLevel {
    Critical,
    High,
    Medium,
    Low,
    Debug,
    Info,
}
