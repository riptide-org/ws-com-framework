use crate::error::Error;

/*
Note: These types could be stack allocated, but the recving buff heap allocates them
anyways, so I'd rather just have a single heap allocation than migrating them to the
stack afterwards. If there is a better way to do this I'm all ears.
*/

//4 bytes representing file id
pub type FileId = Vec<u8>;

//6 bytes representing a server public id
pub type PublicId = Vec<u8>;

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
    use self::fsp_comm::{Auth, AuthReq, Error as CommError, Metadata, Pong, UploadTo};
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

    impl TryFrom<Vec<u8>> for Pong {
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

    impl From<Pong> for FspComm {
        fn from(itm: Pong) -> Self {
            Self {
                r#type: 8,
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

    impl TryFrom<Vec<u8>> for FspComm {
        type Error = super::Error;
        fn try_from(msg: Vec<u8>) -> Result<Self, super::Error> {
            Ok(Self::decode(&msg[..])?)
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
                ExternalMessage::Error(reason) => Ok(CommError { reason }.into()),
                ExternalMessage::UploadTo(file_id, upload_url) => {
                    if file_id.len() != 4 {
                        return Err(super::Error::ByteEncodeError(String::from(
                            "invalid file_id provided",
                        )));
                    }
                    Ok(UploadTo {
                        file_id: file_id,
                        upload_url,
                    }
                    .into())
                }
                ExternalMessage::Metadata(file_id, upload_url) => {
                    if file_id.len() != 4 {
                        return Err(super::Error::ByteEncodeError(String::from(
                            "invalid file_id provided",
                        )));
                    }
                    Ok(Metadata {
                        file_id: file_id,
                        upload_url,
                    }
                    .into())
                }
                ExternalMessage::AuthReq(public_id) => {
                    if public_id.len() != 6 {
                        return Err(super::Error::ByteEncodeError(String::from(
                            "invalid public_id provided",
                        )));
                    }
                    Ok(AuthReq {
                        public_id: public_id,
                    }
                    .into())
                }
                ExternalMessage::AuthRes(public_id, passcode) => {
                    if public_id.len() != 6 {
                        return Err(super::Error::ByteEncodeError(String::from(
                            "invalid public_id provided",
                        )));
                    }
                    if passcode.len() != 32 {
                        return Err(super::Error::ByteEncodeError(String::from(
                            "invalid passcode provided",
                        )));
                    }
                    Ok(Auth {
                        public_id: public_id,
                        passcode: passcode,
                    }
                    .into())
                }
                ExternalMessage::Ping => Ok(Self {
                    r#type: 7,
                    value: Vec::with_capacity(0),
                }),
                ExternalMessage::Pong(status) => Ok(Pong { status }.into()),
            }
        }
    }

    impl TryFrom<FspComm> for ExternalMessage {
        type Error = super::Error;

        fn try_from(value: FspComm) -> Result<Self, super::Error> {
            if let Some(ty) = fsp_comm::Type::from_i32(value.r#type) {
                let res = match ty {
                    fsp_comm::Type::Ok => ExternalMessage::Ok,
                    fsp_comm::Type::Error => {
                        let tmp: CommError = value.value.try_into()?;
                        ExternalMessage::Error(tmp.reason)
                    }
                    fsp_comm::Type::UploadTo => {
                        let tmp: UploadTo = value.value.try_into()?;
                        if tmp.file_id.len() != 4 {
                            return Err(super::Error::ByteDecodeError(String::from(
                                "invalid file_id provided",
                            )));
                        }
                        ExternalMessage::UploadTo(tmp.file_id, tmp.upload_url)
                    }
                    fsp_comm::Type::Metadata => {
                        let tmp: Metadata = value.value.try_into()?;
                        if tmp.file_id.len() != 4 {
                            return Err(super::Error::ByteDecodeError(String::from(
                                "invalid file_id provided",
                            )));
                        }
                        ExternalMessage::Metadata(tmp.file_id, tmp.upload_url)
                    }
                    fsp_comm::Type::Authreq => {
                        let tmp: AuthReq = value.value.try_into()?;
                        if tmp.public_id.len() != 6 {
                            return Err(super::Error::ByteDecodeError(String::from(
                                "invalid public_id provided",
                            )));
                        }
                        ExternalMessage::AuthReq(tmp.public_id)
                    }
                    fsp_comm::Type::Auth => {
                        let tmp: Auth = value.value.try_into()?;
                        if tmp.public_id.len() != 6 {
                            return Err(super::Error::ByteDecodeError(String::from(
                                "invalid public_id provided",
                            )));
                        }
                        if tmp.passcode.len() != 32 {
                            return Err(super::Error::ByteDecodeError(String::from(
                                "invalid passcode provided",
                            )));
                        }
                        ExternalMessage::AuthRes(tmp.public_id, tmp.passcode)
                    }
                    fsp_comm::Type::Ping => ExternalMessage::Ping,
                    fsp_comm::Type::Pong => {
                        let tmp: Pong = value.value.try_into()?;
                        ExternalMessage::Pong(tmp.status)
                    }
                };
                Ok(res)
            } else {
                Err(super::Error::ByteDecodeError(String::from(
                    "unrecognised i32 variant",
                )))
            }
        }
    }
}

enum Message {
    Ok,
    Error(Option<String>),
    UploadTo(FileId, String),
    Metadata(FileId, String),
    AuthReq(PublicId),
    AuthRes(PublicId, Passcode),
    Ping,
    Pong(Option<String>),
}

impl Message {
    /// Attempt to convert the provided type into a valid protobuf3 stream.
    /// Validates that types are of the correct length before conversion.
    pub fn into_bytes(self) -> Result<Vec<u8>, Error> {
        use websocket_message::FspComm;
        let tmp: FspComm = self.try_into()?;
        Ok(into_bytes!(tmp))
    }

    /// Attempt to decode a prost byte stream into this type. Note that the
    /// stream must be encoded using the correct protobuf3 protocols.
    pub fn from_bytes(input: Vec<u8>) -> Result<Self, Error> {
        use websocket_message::FspComm;
        let tmp: FspComm = input.try_into()?;
        Ok(tmp.try_into()?)
    }
}
