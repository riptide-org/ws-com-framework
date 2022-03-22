use crate::error::Error;

pub mod websocket_message {
    use std::convert::Infallible;

    use prost::Message;
    use super::Message as ExternalMessage;
    use self::fsp_comm::{UploadTo, Auth, Pong, Error as CommError, Metadata, AuthReq};

    /// A macro for converting a provided type into bytes for sending over a stream
    macro_rules! into_bytes {
        ($data:tt) => {{
            let mut buf = Vec::with_capacity($data.encoded_len());
            $data
                .encode(&mut buf)
                .expect("Should never fail, as Vec<u8> expands automatically");

            buf
        }};
    }

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

    impl From<ExternalMessage> for FspComm {
        fn from(msg: ExternalMessage) -> Self {
            match msg {
                ExternalMessage::Ok => Self {
                    r#type: 1,
                    value: Vec::with_capacity(0),
                },
                ExternalMessage::Error(reason) => CommError { reason, }.into(),
                ExternalMessage::UploadTo(file_id, upload_url) => UploadTo { file_id: file_id.to_vec(), upload_url }.into(),
                ExternalMessage::Metadata(file_id, upload_url) => Metadata { file_id: file_id.to_vec(), upload_url }.into(),
                ExternalMessage::AuthReq(public_id) => AuthReq {
                    public_id: public_id.to_vec()
                }.into(),
                ExternalMessage::AuthRes(public_id, passcode) => Auth {
                    public_id: public_id.to_vec(),
                    passcode: passcode.to_vec(),
                }.into(),
                ExternalMessage::Ping => Self {
                    r#type: 7,
                    value: Vec::with_capacity(0),
                },
                ExternalMessage::Pong(status) => Pong { status }.into(),
            }
        }
    }

    impl TryFrom<FspComm> for ExternalMessage {
        type Error = super::Error;

        fn try_from(value: FspComm) -> Result<Self, Self::Error> {
            if let Some(ty) = fsp_comm::Type::from_i32(value.r#type) {
                let res = match ty {
                    fsp_comm::Type::Ok => ExternalMessage::Ok,
                    fsp_comm::Type::Error => ExternalMessage::Error(value.value.try_into()?),
                    fsp_comm::Type::UploadTo => ExternalMessage::UploadTo((), ()),
                    fsp_comm::Type::Metadata => ,
                    fsp_comm::Type::Authreq => ,
                    fsp_comm::Type::Auth => ,
                    fsp_comm::Type::Ping => ,
                    fsp_comm::Type::Pong => ,
                };
                Ok(res)
            } else {
                Err(super::Error::ByteDecodeError("unrecognised i32 variant"))
            }
        }
    }
}

enum Message {
    Ok,
    Error(Option<String>),
    UploadTo([u8; 4], String),
    Metadata([u8; 4], String),
    AuthReq([u8; 6]),
    AuthRes([u8; 6], [u8; 32]),
    Ping,
    Pong(Option<String>),
}

impl Message {
    fn into_bytes(self) -> Vec<u8> {
        use websocket_message::FspComm;
        todo!()
    }
}