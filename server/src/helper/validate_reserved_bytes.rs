///Checkea bits reservados del ultimo byte del subscribe
pub fn check_reserved_bytes(byte: u8) -> Result<(), String> {
    let result = byte & 0b00001111 == 0x02;
    match result {
        true => return Ok(()),
        _ => return Err("Malformed Subscribe Error".to_string()),
    }
}