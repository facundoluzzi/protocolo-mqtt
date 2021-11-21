#[cfg(test)]
mod tests {
    use server::variable_header::publish_variable_header::get_variable_header;

    #[test]
    fn obtiene_variable_header_en_publish_correctamente() {
        let vec: &[u8] = &[0x00, 0x03, 0x41, 0x2F, 0x42, 0x00, 0x00];
        let (topic, packet_identifier, length) = get_variable_header(vec);
        assert_eq!(topic, "A/B".to_owned());
        assert_eq!(packet_identifier, [0x00, 0x00]);
        assert_eq!(length, 7);
    }
}