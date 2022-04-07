//! Error handling internally and externally for the ws-com-framework

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Whether the connection should be closed after this error.
pub enum EndOfConnection {
    /// The connection will now close, please flush buffers
    End,
    /// The connection will continue, no need to disconnect
    Continue,
}

impl From<bool> for EndOfConnection {
    fn from(b: bool) -> Self {
        match b {
            true => Self::End,
            false => Self::Continue,
        }
    }
}

impl From<EndOfConnection> for bool {
    fn from(c: EndOfConnection) -> Self {
        match c {
            EndOfConnection::End => true,
            EndOfConnection::Continue => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Represents the kind of error recieved from a peer
pub enum ErrorKind {
    /// The client has sent a unique error that does not
    /// have a predetermined type. They should have set the Option<String> value.
    Unknown = 0,
    /// Requested file was unable to be uploaded
    FailedFileUpload = 1,
    /// Requested file does not exist
    FileDoesntExist = 2,
    /// You should have authenticated before
    InvalidSession = 3,
}

impl From<i32> for ErrorKind {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::FailedFileUpload,
            2 => Self::FileDoesntExist,
            3 => Self::InvalidSession,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// Error types, represents both errors recieved from a peer, and internal conversion errors
/// inside of the framework.
pub enum Error {
    /// Unable to decode recieved message
    ByteDecodeError(String),

    /// Unable to encode provided message to send
    ByteEncodeError(String),
}

impl From<prost::DecodeError> for Error {
    fn from(err: prost::DecodeError) -> Self {
        Self::ByteDecodeError(err.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ByteDecodeError(e) => write!(f, "failed to decode bytes as valid message {}", e),
            Error::ByteEncodeError(e) => write!(f, "failed to encode bytes as valid message {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
