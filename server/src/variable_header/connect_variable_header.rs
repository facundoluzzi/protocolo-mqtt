pub fn get_keep_alive(variable_header: &[u8]) -> Option<u8> {
    let msb_keep_alive = variable_header[8];
    let lsb_keep_alive = variable_header[9];

    if msb_keep_alive == 0x00 && lsb_keep_alive == 0x00 {
        None
    } else if msb_keep_alive > lsb_keep_alive {
        Some(msb_keep_alive)
    } else {
        Some(lsb_keep_alive)
    }
}

pub fn check_variable_header_len(variable_header: &[u8]) -> Result<String, String> {
    if variable_header.len() != 10 {
        Err("Invalid variable header length".to_string())
    } else {
        Ok("OK".to_string())
    }
}