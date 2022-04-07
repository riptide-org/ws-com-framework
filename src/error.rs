#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    /* Communication/parsing failures inside of ws com */

    /// Unable to decode recieved message
    ByteDecodeError(String),

    /// Unable to encode provided message to send
    ByteEncodeError(String),

    /// Unable to send a message
    SendFailure(String),

    /// Unable to recieve a message
    ReceiveFailure(String),

    /// Unable to close socket
    //XXX: could this be made into an std::io::error??
    CloseFailure(String),

    /* Following errors can be send/recieved from a client */

    /// Requested file was unable to be uploaded
    FailedFileUpload(Option<String>, EndOfConnection),

    /// Requested file does not exist
    FileDoesntExist(Option<String>, EndOfConnection),

    /// You should have authenticated before
    InvalidSession(Option<String>, EndOfConnection),

    /// The client has sent a unique error that does not
    /// have a predetermined type. They should have set the Option<String> value.
    Unknown(Option<String>, EndOfConnection),
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
            Error::Unknown(_, _) => todo!(),
            Error::SendFailure(_) => todo!(),
            Error::ReceiveFailure(_) => todo!(),
            Error::CloseFailure(_) => todo!(),

            Error::FailedFileUpload(_, _) => todo!(),
            Error::FileDoesntExist(_, _) => todo!(),
            Error::InvalidSession(_, _) => todo!(),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None

        // match *self {
        //     Error::ByteDecodeError(_) => todo!(),
        //     Error::ByteEncodeError(_) => todo!(),
        //     // Error::SendFailure(snd_fl) => Some(snd_fl),
        //     Error::FailedFileUpload(_) => todo!(),
        //     Error::FileDoesntExist(_) => todo!(),
        //     Error::InvalidSession(_) => todo!(),
        //     Error::Unknown(_) => todo!(),
        // }
    }
}
