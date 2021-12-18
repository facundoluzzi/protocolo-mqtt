use server::helper::utf8_parser::UTF8;

#[test]
fn should_parse_msb() {
    let vec: &[u8] = &[0x02, 0x00, 0x41, 0x42, 0x45, 0x4D];
    let (string_result, length) = UTF8::utf8_parser(vec).unwrap();
    assert_eq!(string_result, "BA".to_owned());
    assert_eq!(length, 4);
}

#[test]
fn should_parse_lsb() {
    let vec: &[u8] = &[0x00, 0x02, 0x41, 0x42];
    let (string_result, length) = UTF8::utf8_parser(vec).unwrap();
    assert_eq!(string_result, "AB".to_owned());
    assert_eq!(length, 4);
}

#[test]
fn should_fail_when_vec_is_short() {
    let vec: &[u8] = &[];
    if let Err(err_message) = UTF8::utf8_parser(vec) {
        assert_eq!(err_message, "Length should be at least 3".to_string());
    } else {
        assert_eq!(true, false);
    }
}

#[test]
fn should_fail_when_encoding_isnot_correct() {
    let vec: &[u8] = &[0x00, 0x02, 0x41];
    if let Err(err_message) = UTF8::utf8_parser(vec) {
        assert_eq!(err_message, "Malformed code".to_string());
    } else {
        assert_eq!(true, false);
    }
}
