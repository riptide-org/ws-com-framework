#[derive(Debug, Clone)]
pub enum Error {
    ByteDecodeError(String),
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
