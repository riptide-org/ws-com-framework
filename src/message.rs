use crate::error::Error;

/*
Note: These types could be stack allocated, but the recving buff heap allocates them
anyways, so I'd rather just have a single heap allocation than migrating them to the
stack afterwards. If there is a better way to do this I'm all ears.
*/

//4 bytes representing file id
pub type FileId = u32;

//8 bytes representing a server public id
pub type PublicId = u64;

//32 byte authentication key
pub type Passcode = Vec<u8>;

/// A macro for converting a provided type into bytes for sending over a stream
macro_rules! into_bytes {
    ($data:tt) => {{
        use prost::Message;
        let mut buf = Vec::with_capacity($data.encoded_len());
        $data
            .encode(&mut buf)
            .expect("Should never fail, as Vec<u8> expands automatically");

        buf
    }};
}

pub mod websocket_message {
    use self::fsp_comm::{Auth, AuthReq, Error as CommError, Metadata, UploadTo};
    use super::Message as ExternalMessage;
    use prost::Message;

    include!(concat!(env!("OUT_DIR"), "/events.rs"));

    impl TryFrom<Vec<u8>> for UploadTo {
        type Error = super::Error;
        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(Self::decode(&value[..])?)
        }
    }

    impl TryFrom<Vec<u8>> for Metadata {
        type Error = super::Error;
        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(Self::decode(&value[..])?)
        }
    }

    impl TryFrom<Vec<u8>> for AuthReq {
        type Error = super::Error;
        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(Self::decode(&value[..])?)
        }
    }

    impl TryFrom<Vec<u8>> for Auth {
        type Error = super::Error;
        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(Self::decode(&value[..])?)
        }
    }

    impl TryFrom<Vec<u8>> for CommError {
        type Error = super::Error;
        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(Self::decode(&value[..])?)
        }
    }

    impl From<UploadTo> for FspComm {
        fn from(itm: UploadTo) -> Self {
            Self {
                r#type: 3,
                value: into_bytes!(itm),
            }
        }
    }

    impl From<Metadata> for FspComm {
        fn from(itm: Metadata) -> Self {
            Self {
                r#type: 4,
                value: into_bytes!(itm),
            }
        }
    }

    impl From<AuthReq> for FspComm {
        fn from(itm: AuthReq) -> Self {
            Self {
                r#type: 5,
                value: into_bytes!(itm),
            }
        }
    }

    impl From<Auth> for FspComm {
        fn from(itm: Auth) -> Self {
            Self {
                r#type: 6,
                value: into_bytes!(itm),
            }
        }
    }

    impl From<CommError> for FspComm {
        fn from(itm: CommError) -> Self {
            Self {
                r#type: 1,
                value: into_bytes!(itm),
            }
        }
    }

    impl TryFrom<&[u8]> for FspComm {
        type Error = super::Error;
        fn try_from(msg: &[u8]) -> Result<Self, super::Error> {
            Ok(Self::decode(msg)?)
        }
    }

    impl TryFrom<ExternalMessage> for FspComm {
        type Error = super::Error;
        fn try_from(msg: ExternalMessage) -> Result<Self, super::Error> {
            match msg {
                ExternalMessage::Ok => Ok(Self {
                    r#type: 1,
                    value: Vec::with_capacity(0),
                }),
                ExternalMessage::Error(err) => {
                    let r#type = match err {
                        crate::Error::FailedFileUpload(_, _) => 1,
                        crate::Error::FileDoesntExist(_, _) => 2,
                        crate::Error::InvalidSession(_, _) => 3,
                        _ => 0,
                    };

                    match err {
                        crate::Error::FailedFileUpload(reason, connection_end) |
                        crate::Error::FileDoesntExist(reason, connection_end) |
                        crate::Error::InvalidSession(reason, connection_end) => Ok(CommError {
                            r#type,
                            connection_end: connection_end.into(),
                            reason,
                        }.into()),
                        e => Ok(CommError {
                            r#type: 0,
                            connection_end: false,
                            reason: Some(e.to_string()),
                        }.into()),
                    }
                },
                ExternalMessage::UploadTo(file_id, upload_url) => {
                    Ok(UploadTo {
                        file_id: file_id,
                        upload_url,
                    }
                    .into())
                },
                ExternalMessage::Metadata(file_id, upload_url) => {
                    Ok(Metadata {
                        file_id: file_id,
                        upload_url,
                    }
                    .into())
                },
                ExternalMessage::AuthReq(public_id) => {
                    Ok(AuthReq {
                        public_id: public_id,
                    }
                    .into())
                },
                ExternalMessage::AuthRes(public_id, passcode) => {
                    Ok(Auth {
                        public_id: public_id,
                        passcode: passcode,
                    }
                    .into())
                },
                ExternalMessage::Close => todo!(),
            }
        }
    }

    impl TryFrom<FspComm> for ExternalMessage {
        type Error = super::Error;

        fn try_from(value: FspComm) -> Result<Self, super::Error> {
            if let Some(ty) = fsp_comm::Type::from_i32(value.r#type) {
                match ty {
                    fsp_comm::Type::Ok => Ok(ExternalMessage::Ok),
                    fsp_comm::Type::Error => {
                        let tmp: CommError = value.value.try_into()?;
                        match tmp.r#type {
                            0 => Ok(Self::Error(crate::error::Error::Unknown(tmp.reason, tmp.connection_end.into()))),
                            1 => Ok(Self::Error(crate::error::Error::FailedFileUpload(tmp.reason, tmp.connection_end.into()))),
                            2 => Ok(Self::Error(crate::error::Error::FileDoesntExist(tmp.reason, tmp.connection_end.into()))),
                            3 => Ok(Self::Error(crate::error::Error::InvalidSession(tmp.reason, tmp.connection_end.into()))),
                            _ => Err(super::Error::ByteEncodeError(String::from("invalid error type recieved"))),
                        }
                    },
                    fsp_comm::Type::UploadTo => {
                        let tmp: UploadTo = value.value.try_into()?;
                        Ok(ExternalMessage::UploadTo(tmp.file_id, tmp.upload_url))
                    },
                    fsp_comm::Type::Metadata => {
                        let tmp: Metadata = value.value.try_into()?;
                        Ok(ExternalMessage::Metadata(tmp.file_id, tmp.upload_url))
                    },
                    fsp_comm::Type::Authreq => {
                        let tmp: AuthReq = value.value.try_into()?;
                        Ok(ExternalMessage::AuthReq(tmp.public_id))
                    },
                    fsp_comm::Type::Auth => {
                        let tmp: Auth = value.value.try_into()?;
                        Ok(ExternalMessage::AuthRes(tmp.public_id, tmp.passcode))
                    },
                }
            } else {
                Err(super::Error::ByteDecodeError(String::from(
                    "unrecognised i32 variant",
                )))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Ok,
    Error(Error),
    UploadTo(FileId, String),
    Metadata(FileId, String),
    AuthReq(PublicId),
    AuthRes(PublicId, Passcode),
    Close,
}

impl Message {
    /// Attempt to convert the provided type into a valid protobuf3 strestaticm.
    /// Validates that types are of the correct length before conversion.
    pub fn into_bytes(self) -> Result<Vec<u8>, Error> {
        use websocket_message::FspComm;
        let tmp: FspComm = self.try_into()?;
        Ok(into_bytes!(tmp))
    }

    /// Attempt to decode a prost byte stream into this type. Note that the
    /// stream must be encoded using the correct protobuf3 protocols.
    pub fn from_bytes(input: &[u8]) -> Result<Self, Error> {
        use websocket_message::FspComm;
        let tmp: FspComm = input.try_into()?;
        Ok(tmp.try_into()?)
    }
}
