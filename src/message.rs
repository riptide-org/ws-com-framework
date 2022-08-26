//! Messages handles provides the Message type.
//!
//! Internally it also provides conversions between the Message type to/from bytes.

use crate::error::{Error, ErrorKind};

/*
Note: These types could be stack allocated, but the receiving buff heap allocates them
anyways, so I'd rather just have a single heap allocation than migrating them to the
stack afterwards. If there is a better way to do this I'm all ears.
*/

/// 4 bytes representing file id
pub type FileId = u32;

/// 8 bytes representing a server public id
pub type PublicId = u64;

/// 32 byte authentication key
pub type Passcode = Vec<u8>;

/// 8 bytes representing a public upload id on the server
pub type UploadId = u64;

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

/// `websocket_message` segregates the internal message type used by protobuf3
pub mod websocket_message {
    use crate::error::ErrorKind;

    use self::protobuf_types::fsp_comm::{
        Auth, AuthReq, Error as CommError, MetadataReq, MetadataRes, UploadTo,
    };
    use self::protobuf_types::fsp_comm::{StatusReq, StatusRes};
    use self::protobuf_types::FspComm;
    use super::Message as ExternalMessage;
    use prost::Message;

    #[allow(clippy::all, missing_docs, missing_copy_implementations)]
    #[cfg(not(tarpaulin_include))]
    pub mod protobuf_types {
        include!(concat!(env!("OUT_DIR"), "/events.rs"));
    }

    impl TryFrom<Vec<u8>> for UploadTo {
        type Error = super::Error;
        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(Self::decode(&value[..])?)
        }
    }

    impl TryFrom<Vec<u8>> for MetadataReq {
        type Error = super::Error;
        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(Self::decode(&value[..])?)
        }
    }

    impl TryFrom<Vec<u8>> for MetadataRes {
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

    impl TryFrom<Vec<u8>> for StatusRes {
        type Error = super::Error;
        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(Self::decode(&value[..])?)
        }
    }

    impl TryFrom<Vec<u8>> for StatusReq {
        type Error = super::Error;
        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(Self::decode(&value[..])?)
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

    impl From<UploadTo> for FspComm {
        fn from(itm: UploadTo) -> Self {
            Self {
                r#type: 2,
                value: into_bytes!(itm),
            }
        }
    }

    impl From<MetadataReq> for FspComm {
        fn from(itm: MetadataReq) -> Self {
            Self {
                r#type: 3,
                value: into_bytes!(itm),
            }
        }
    }

    impl From<MetadataRes> for FspComm {
        fn from(itm: MetadataRes) -> Self {
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

    impl From<StatusReq> for FspComm {
        fn from(value: StatusReq) -> Self {
            Self {
                r#type: 7,
                value: into_bytes!(value),
            }
        }
    }

    impl From<StatusRes> for FspComm {
        fn from(value: StatusRes) -> Self {
            Self {
                r#type: 8,
                value: into_bytes!(value),
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
                    r#type: 0,
                    value: Vec::with_capacity(0),
                }),
                ExternalMessage::Error { kind, reason } => Ok(CommError {
                    r#type: kind as i32,
                    reason,
                }
                .into()),
                ExternalMessage::UploadTo {
                    file_id,
                    upload_url,
                } => Ok(UploadTo {
                    file_id,
                    upload_url,
                }
                .into()),
                ExternalMessage::MetadataReq { file_id, upload_id } => {
                    Ok(MetadataReq { file_id, upload_id }.into())
                }
                ExternalMessage::MetadataRes {
                    file_id,
                    exp,
                    crt,
                    file_size,
                    username,
                    file_name,
                    upload_id,
                } => Ok(MetadataRes {
                    file_id,
                    exp,
                    crt,
                    file_size,
                    username,
                    file_name,
                    upload_id,
                }
                .into()),
                ExternalMessage::AuthReq { public_id } => Ok(AuthReq { public_id }.into()),
                ExternalMessage::AuthRes {
                    public_id,
                    passcode,
                } => Ok(Auth {
                    public_id,
                    passcode,
                }
                .into()),
                ExternalMessage::StatusReq { public_id } => Ok(StatusReq { public_id }.into()),
                ExternalMessage::StatusRes {
                    public_id,
                    ready,
                    uptime,
                    message,
                } => Ok(StatusRes {
                    public_id,
                    ready,
                    uptime,
                    message,
                }
                .into()),
            }
        }
    }

    impl TryFrom<FspComm> for ExternalMessage {
        type Error = super::Error;
        fn try_from(value: FspComm) -> Result<Self, super::Error> {
            if let Some(ty) = protobuf_types::fsp_comm::Type::from_i32(value.r#type) {
                match ty {
                    protobuf_types::fsp_comm::Type::Ok => Ok(ExternalMessage::Ok),
                    protobuf_types::fsp_comm::Type::Error => {
                        let tmp: CommError = value.value.try_into()?;
                        Ok(ExternalMessage::Error {
                            kind: ErrorKind::from(tmp.r#type),
                            reason: tmp.reason,
                        })
                    }
                    protobuf_types::fsp_comm::Type::UploadTo => {
                        let tmp: UploadTo = value.value.try_into()?;
                        Ok(ExternalMessage::UploadTo {
                            file_id: tmp.file_id,
                            upload_url: tmp.upload_url,
                        })
                    }
                    protobuf_types::fsp_comm::Type::MetadataReq => {
                        let tmp: MetadataReq = value.value.try_into()?;
                        Ok(ExternalMessage::MetadataReq {
                            file_id: tmp.file_id,
                            upload_id: tmp.upload_id,
                        })
                    }
                    protobuf_types::fsp_comm::Type::MetadataRes => {
                        let tmp: MetadataRes = value.value.try_into()?;
                        Ok(ExternalMessage::MetadataRes {
                            file_id: tmp.file_id,
                            exp: tmp.exp,
                            crt: tmp.crt,
                            file_size: tmp.file_size,
                            username: tmp.username,
                            file_name: tmp.file_name,
                            upload_id: tmp.upload_id,
                        })
                    }
                    protobuf_types::fsp_comm::Type::Authreq => {
                        let tmp: AuthReq = value.value.try_into()?;
                        Ok(ExternalMessage::AuthReq {
                            public_id: tmp.public_id,
                        })
                    }
                    protobuf_types::fsp_comm::Type::Auth => {
                        let tmp: Auth = value.value.try_into()?;
                        Ok(ExternalMessage::AuthRes {
                            public_id: tmp.public_id,
                            passcode: tmp.passcode,
                        })
                    }
                    protobuf_types::fsp_comm::Type::StatusReq => {
                        let tmp: StatusReq = value.value.try_into()?;
                        Ok(ExternalMessage::StatusReq {
                            public_id: tmp.public_id,
                        })
                    }
                    protobuf_types::fsp_comm::Type::StatusRes => {
                        let tmp: StatusRes = value.value.try_into()?;
                        Ok(ExternalMessage::StatusRes {
                            public_id: tmp.public_id,
                            ready: tmp.ready,
                            uptime: tmp.uptime,
                            message: tmp.message,
                        })
                    }
                }
            } else {
                Err(super::Error::ByteDecodeError(String::from(
                    "unrecognised i32 variant",
                )))
            }
        }
    }
}

/// The Message type is a piece of data that can be sent between a peer and client
/// it is designed to be send through a websocket connection, and is converted to
/// protobuf3 to facilitate this sending.
/// Recieved messages are converted from binary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    /// Acknowledgement of a previous response
    Ok,
    /// An error has occurred, expect this in response to sending a bad request
    Error {
        /// The kind of error that has occurred
        kind: ErrorKind,
        /// A human readable reason for the error, optionally included
        reason: Option<String>,
    },
    /// Request the peer to upload the provided `FileId` to the provided url
    UploadTo {
        /// The `FileId` of the file to upload
        file_id: FileId,
        /// The url that the file should be POSTed to in a streaming fashion
        upload_url: String,
    },
    /// Reuqest the peer to upload the provided `FileId` metadata
    MetadataReq {
        /// The `FileId` of the file to upload metadata from
        file_id: FileId,
        /// The upload_id to attach when returning with MetadataRes
        upload_id: UploadId,
    },
    /// The metadata about a share sent from an agent
    MetadataRes {
        /// Unique id for this file type
        file_id: u32,
        /// Time when this share will expire, in seconds past epoch
        exp: u64,
        /// Time when this share was created, in seconds past epoch
        crt: u64,
        /// File_size of the share in bytes
        file_size: u64,
        /// Username of the person who shared the field
        username: String,
        /// Name of hte file
        file_name: String,
        /// The id of this file upload
        upload_id: UploadId,
    },
    /// Request this peer to authenticate itself using the `PublicId` provided.
    AuthReq {
        /// The `PublicId` of the peer to authenticate
        public_id: PublicId,
    },
    /// Response from peer with the `PublicId` it is attempting to authenticate
    /// and the associated `Passcode` for that `PublicId`.
    AuthRes {
        /// The `PublicId` of the peer being authenticated
        public_id: PublicId,
        /// The `Passcode` for the `PublicId` being authenticated
        passcode: Passcode,
    },
    /// Request the status of the peer, which should be returned in the form of `Message::StatusReq`
    /// containing a StatusData struct
    StatusReq {
        /// The `PublicId` of the peer to request status from
        public_id: PublicId,
    },
    /// Response to a `Message::StatusReq` containing the status of the peer
    StatusRes {
        /// Unique id for this peer
        public_id: u64,
        /// Whether the peer is ready to accept connections
        ready: bool,
        /// Uptime of the peer in seconds
        uptime: u64,
        /// Optional uptime message from the peer
        message: Option<String>,
    },
}

impl Message {
    /// Attempt to convert the provided type into a valid protobuf3 strestaticm.
    /// Validates that types are of the correct length before conversion.
    #[deprecated(since = "1.0.0", note = "please use `TryFrom` instead")]
    pub fn into_bytes(self) -> Result<Vec<u8>, Error> {
        use websocket_message::protobuf_types::FspComm;
        let tmp: FspComm = self.try_into()?;
        Ok(into_bytes!(tmp))
    }

    /// Attempt to decode a prost byte stream into this type. Note that the
    /// stream must be encoded using the correct protobuf3 protocols.
    #[deprecated(since = "1.0.0", note = "please use `TryFrom` instead")]
    pub fn from_bytes(input: &[u8]) -> Result<Self, Error> {
        use websocket_message::protobuf_types::FspComm;
        let tmp: FspComm = input.try_into()?;
        tmp.try_into()
    }
}

impl TryFrom<Vec<u8>> for Message {
    type Error = Error;
    #[allow(deprecated)] //TEMP
    fn try_from(value: Vec<u8>) -> Result<Self, Error> {
        Self::from_bytes(&value[..])
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self, Error> {
        #[allow(deprecated)] //TEMP
        Self::from_bytes(value)
    }
}

impl TryFrom<Message> for Vec<u8> {
    type Error = Error;
    #[allow(deprecated)] //TEMP
    fn try_from(value: Message) -> Result<Self, Error> {
        value.into_bytes()
    }
}
