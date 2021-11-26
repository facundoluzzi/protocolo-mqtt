#[cfg(test)]
mod tests {
    use server::variable_header::publish_variable_header::get_variable_header;
    use server::variable_header::publish_variable_header::verify_publish_wilcard;

    #[test]
    fn should_get_variable_header_in_publish_successfully() {
        let vec: &[u8] = &[0x00, 0x03, 0x41, 0x2F, 0x42, 0x00, 0x00];
        let (topic, packet_identifier, length) = get_variable_header(vec).unwrap();
        assert_eq!(topic, "A/B".to_owned());
        assert_eq!(packet_identifier, [0x00, 0x00]);
        assert_eq!(length, 7);
    }

    #[test]
    fn should_verify_topic_successfully() {
        let topic = "A/B".to_string();
        let is_valid = verify_publish_wilcard(topic);
        assert_eq!(is_valid, true);
    }    
}
