pub fn get_variable_header(bytes: &[u8]) -> Result<(&[u8], usize), String> {
    match bytes.get(2) {
        None => Err("Invalid header length".to_string()),
        Some(_) => Ok((&bytes[0..2], 2)),
    }
}
