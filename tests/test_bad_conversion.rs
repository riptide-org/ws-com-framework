use ws_com_framework::{Error, Message};

#[test]
fn test_bad_type_conversion() {
    let bytes: Vec<u8> = vec![
        0, 1, 18, 28, 8, 1, 26, 24, 117, 110, 97, 98, 108, 101, 32, 116, 111, 32, 118, 97, 108,
        105, 100, 32, 105, 110, 115, 116, 97, 108, 108, 49,
    ];
    let msg2: Result<Message, Error> = Message::try_from(bytes);

    assert!(msg2.is_err());
    assert!(msg2
        .unwrap_err()
        .to_string()
        .contains("failed to decode bytes as valid message"));

    let bytes: Vec<u8> = vec![8, 10, 18, 7, 10, 5, 104, 101, 108, 108, 111];
    let msg2: Result<Message, Error> = Message::try_from(bytes);

    assert!(msg2.is_err());
    let msg2 = msg2.unwrap_err();

    assert!(msg2
        .to_string()
        .contains("failed to decode bytes as valid message"));
    assert!(msg2.to_string().contains("unrecognised i32 variant"));
}
