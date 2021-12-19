use server::variable_header::connect_variable_header::{check_variable_header_len, get_keep_alive};

#[test]
fn should_return_a_valid_keep_alive_lsb() {
    let vec: [u8; 10] = [0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x00, 0x00, 0x00, 0x03];
    match get_keep_alive(&vec) {
        Some(keep_alive) => assert_eq!(keep_alive, 0x03),
        None => panic!("should return Some(0x03)"),
    };
}

#[test]
fn should_return_a_valid_keep_alive_msb() {
    let vec: [u8; 10] = [0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x00, 0x00, 0x04, 0x03];
    match get_keep_alive(&vec) {
        Some(keep_alive) => assert_eq!(keep_alive, 0x04),
        None => panic!("should return Some(0x03)"),
    };
}

#[test]
fn should_return_none() {
    let vec: [u8; 10] = [0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x00, 0x00, 0x00, 0x00];
    match get_keep_alive(&vec) {
        Some(_keep_alive) => panic!("should return None"),
        None => true,
    };
}

#[test]
fn should_return_ok_variable_header_len() {
    let vec: [u8; 10] = [0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x00, 0x00, 0x00, 0x03];
    match check_variable_header_len(&vec) {
        Ok(msg) => assert_eq!("OK".to_string(), msg),
        Err(_msg) => panic!("should return Ok"),
    };
}

#[test]
fn should_return_err_variable_header_len_when_contains_more() {
    let vec: [u8; 11] = [
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x00, 0x00, 0x00, 0x03, 0x00,
    ];
    match check_variable_header_len(&vec) {
        Ok(_msg) => panic!("should fail"),
        Err(msg) => assert_eq!(msg, "Invalid variable header length".to_string()),
    };
}
