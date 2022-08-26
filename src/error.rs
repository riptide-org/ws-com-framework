//! Error handling internally and externally for the ws-com-framework

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Represents the kind of error received from a peer
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
/// Error types, represents both errors received from a peer, and internal conversion errors inside of the framework.
pub enum Error {
    /// Unable to decode received message
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

impl std::error::Error for Error {}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod test_err {
    #[test]
    fn test_printing_errors() {
        let err = super::Error::ByteDecodeError(String::from("test"));
        assert_eq!(
            format!("{}", err),
            "failed to decode bytes as valid message test"
        );

        let err = super::Error::ByteEncodeError(String::from("test"));
        assert_eq!(
            format!("{}", err),
            "failed to encode bytes as valid message test"
        );
    }
}
