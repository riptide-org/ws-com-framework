//! Test creating and converting every variant of the `Message` enum.

use ws_com_framework::Message;

/// Test creating and parsing the OK message variant of `Message`.
#[test]
fn test_converting_ok() {
    let msg = Message::Ok {};
    let bytes: Vec<u8> = msg.clone().try_into().unwrap();
    let msg2: Message = Message::try_from(bytes).unwrap();
    assert_eq!(msg, msg2);
}

#[test]
fn test_converting_error() {
    let msg = Message::Error {
        kind: ws_com_framework::error::ErrorKind::FailedFileUpload,
        reason: Some(String::from("unable to valid install1")),
    };
    let bytes: Vec<u8> = msg.clone().try_into().unwrap();
    let msg2: Message = Message::try_from(bytes).unwrap();
    assert_eq!(msg, msg2);

    let msg = Message::Error {
        kind: ws_com_framework::error::ErrorKind::FileDoesntExist,
        reason: Some(String::from("unable to valid install2")),
    };
    let bytes: Vec<u8> = msg.clone().try_into().unwrap();
    let msg2: Message = Message::try_from(bytes).unwrap();
    assert_eq!(msg, msg2);

    let msg = Message::Error {
        kind: ws_com_framework::error::ErrorKind::InvalidSession,
        reason: Some(String::from("unable to valid install3")),
    };
    let bytes: Vec<u8> = msg.clone().try_into().unwrap();
    let msg2: Message = Message::try_from(bytes).unwrap();
    assert_eq!(msg, msg2);

    let msg = Message::Error {
        kind: ws_com_framework::error::ErrorKind::Unknown,
        reason: Some(String::from("unable to valid install4")),
    };
    let bytes: Vec<u8> = msg.clone().try_into().unwrap();
    let msg2: Message = Message::try_from(bytes).unwrap();
    assert_eq!(msg, msg2);
}

#[test]
fn test_converting_upload_to() {
    let msg = Message::UploadTo {
        file_id: 123,
        upload_url: String::from("https://example.com/upload"),
    };
    let bytes: Vec<u8> = msg.clone().try_into().unwrap();
    let msg2: Message = Message::try_from(bytes).unwrap();
    assert_eq!(msg, msg2);
}

#[test]
fn test_converting_metadata_req() {
    let msg = Message::MetadataReq {
        upload_id: 1234,
        file_id: 1234,
    };
    let bytes: Vec<u8> = msg.clone().try_into().unwrap();
    let msg2: Message = Message::try_from(bytes).unwrap();
    assert_eq!(msg, msg2);
}

#[test]
fn test_converting_metadata_res() {
    let msg = Message::MetadataRes {
        file_id: 12343,
        exp: 1234,
        crt: 13834,
        file_size: 34014,
        username: String::from("hello, world"),
        file_name: String::from("hello.txt"),
        upload_id: 123434199,
    };
    let bytes: Vec<u8> = msg.clone().try_into().unwrap();

    let msg2: Message = Message::try_from(bytes).unwrap();
    assert_eq!(msg, msg2);
}

#[test]
fn test_converting_auth_req() {
    let msg = Message::AuthReq {
        public_id: 102983984675,
    };
    let bytes: Vec<u8> = msg.clone().try_into().unwrap();
    let msg2: Message = Message::try_from(bytes).unwrap();
    assert_eq!(msg, msg2);
}

#[test]
fn test_converting_auth_res() {
    let msg = Message::AuthRes {
        public_id: 123087497859,
        passcode: String::from("eraljkafe2123").into_bytes(),
    };
    let bytes: Vec<u8> = msg.clone().try_into().unwrap();
    let msg2: Message = Message::try_from(bytes).unwrap();
    assert_eq!(msg, msg2);
}

#[test]
fn test_converting_status_req() {
    let msg = Message::StatusReq {
        public_id: 12308749783359,
    };
    let bytes: Vec<u8> = msg.clone().try_into().unwrap();
    let msg2: Message = Message::try_from(bytes).unwrap();
    assert_eq!(msg, msg2);
}

#[test]
fn test_converting_status_res() {
    let msg = Message::StatusRes {
        public_id: 123031803797834,
        ready: true,
        uptime: 123,
        message: Some(String::from("ooga buuga my booga")),
    };
    let bytes: Vec<u8> = msg.clone().try_into().unwrap();
    let msg2: Message = Message::try_from(bytes).unwrap();
    assert_eq!(msg, msg2);
}
