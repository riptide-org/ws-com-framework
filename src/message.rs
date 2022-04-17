//! Messages handles provides the Message type.
//!
//! Internally it also provides conversions between the Message type to/from bytes.

use crate::error::{Error, ErrorKind};

/*
Note: These types could be stack allocated, but the recving buff heap allocates them
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
#[allow(missing_docs, missing_copy_implementations)]
pub mod websocket_message {
    use self::fsp_comm::{Auth, AuthReq, Error as CommError, MetadataReq, MetadataRes, UploadTo};
    use super::Message as ExternalMessage;
    use super::ShareMetadata;
    use prost::Message;

    include!(concat!(env!("OUT_DIR"), "/events.rs"));

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
                ExternalMessage::Error(reason, error_kind) => Ok(CommError {
                    r#type: error_kind as i32,
                    reason,
                }
                .into()),
                ExternalMessage::UploadTo(file_id, upload_url) => Ok(UploadTo {
                    file_id,
                    upload_url,
                }
                .into()),
                ExternalMessage::MetadataReq(file_id, upload_id) => {
                    Ok(MetadataReq { file_id, upload_id }.into())
                }
                ExternalMessage::MetadataRes(metadata, upload_id) => Ok(MetadataRes {
                    file_id: metadata.file_id,
                    exp: metadata.exp,
                    crt: metadata.crt,
                    file_size: metadata.file_size,
                    username: metadata.username,
                    file_name: metadata.file_name,
                    upload_id,
                }
                .into()),
                ExternalMessage::AuthReq(public_id) => Ok(AuthReq { public_id }.into()),
                ExternalMessage::AuthRes(public_id, passcode) => Ok(Auth {
                    public_id,
                    passcode,
                }
                .into()),
                ExternalMessage::Close => Err(super::Error::ByteEncodeError(String::from(
                    "don't attempt to convert close type to bytes - this could panic in future",
                ))),
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
                        Ok(ExternalMessage::Error(tmp.reason, tmp.r#type.into()))
                    }
                    fsp_comm::Type::UploadTo => {
                        let tmp: UploadTo = value.value.try_into()?;
                        Ok(ExternalMessage::UploadTo(tmp.file_id, tmp.upload_url))
                    }
                    fsp_comm::Type::MetadataReq => {
                        let tmp: MetadataReq = value.value.try_into()?;
                        Ok(ExternalMessage::MetadataReq(tmp.file_id, tmp.upload_id))
                    }
                    fsp_comm::Type::MetadataRes => {
                        let tmp: MetadataRes = value.value.try_into()?;
                        Ok(ExternalMessage::MetadataRes(
                            ShareMetadata {
                                file_id: tmp.file_id,
                                exp: tmp.exp,
                                crt: tmp.crt,
                                file_size: tmp.file_size,
                                username: tmp.username,
                                file_name: tmp.file_name,
                            },
                            tmp.upload_id,
                        ))
                    }
                    fsp_comm::Type::Authreq => {
                        let tmp: AuthReq = value.value.try_into()?;
                        Ok(ExternalMessage::AuthReq(tmp.public_id))
                    }
                    fsp_comm::Type::Auth => {
                        let tmp: Auth = value.value.try_into()?;
                        Ok(ExternalMessage::AuthRes(tmp.public_id, tmp.passcode))
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

/// The Share type represents a file that has been shared by an agent
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ShareMetadata {
    /// Unique id for this file type
    pub file_id: u32,
    /// Time when this share will expire, in seconds past epoch
    pub exp: u64,
    /// Time when this share was created, in seconds past epoch
    pub crt: u64,
    /// File_size of the share in bytes
    pub file_size: u64,
    /// Username of the person who shared the fiel
    pub username: String,
    /// Name of hte file
    pub file_name: String,
}

/// The Message type is a piece of data that can be sent between a peer and client
/// it is designed to be send through a websocket connection, and is converted to
/// protobuf3 to faciliate this sending.
/// Recieved messages are converted from binary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    /// Acknowledgement of a previous response
    Ok,
    /// An error has occured, expect this in response to sending a bad request
    Error(Option<String>, ErrorKind),
    /// Request the peer to upload the provided `FileId` to the provided url
    UploadTo(FileId, String),
    /// Reuqest the peer to upload the provided `FileId` metadata
    MetadataReq(FileId, UploadId),
    /// The metadata about a share sent from an agent
    MetadataRes(ShareMetadata, UploadId),
    /// Request this peer to authenticate itself using the `PublicId` provided.
    AuthReq(PublicId),
    /// Response from peer with the `PublicId` it is attempting to authenticate
    /// and the associated `Passcode` for that `PublidId`.
    AuthRes(PublicId, Passcode),
    /// The peer has indicated that the connection will close
    Close,
}

impl Message {
    /// Attempt to convert the provided type into a valid protobuf3 strestaticm.
    /// Validates that types are of the correct length before conversion.
    #[deprecated(since = "1.0.0", note = "please use `TryFrom` instead")]
    pub fn into_bytes(self) -> Result<Vec<u8>, Error> {
        use websocket_message::FspComm;
        let tmp: FspComm = self.try_into()?;
        Ok(into_bytes!(tmp))
    }

    /// Attempt to decode a prost byte stream into this type. Note that the
    /// stream must be encoded using the correct protobuf3 protocols.
    #[deprecated(since = "1.0.0", note = "please use `TryFrom` instead")]
    pub fn from_bytes(input: &[u8]) -> Result<Self, Error> {
        use websocket_message::FspComm;
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
