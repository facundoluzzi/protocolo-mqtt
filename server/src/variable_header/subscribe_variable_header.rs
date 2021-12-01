pub fn get_variable_header(bytes: &[u8]) -> (&[u8], usize) {
    let packet_identifier = &bytes[0..2];
    (packet_identifier, 2)
}
