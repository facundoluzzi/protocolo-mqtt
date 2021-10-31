use crate::helper::utf8_parser::UTF8;
/**
 * https://docs.solace.com/PubSub-Basics/SMF-Topics.htm
 */
pub fn get_variable_header(bytes: &[u8]) -> (String, &[u8], usize) {
    let (parsed_topic, readed_bytes) = UTF8::utf8_parser(bytes);
    let packet_identifier = &bytes[readed_bytes + 2..readed_bytes + 4];
    (parsed_topic, packet_identifier, readed_bytes + 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obtiene_variable_header_en_publish_correctamente() {
        let vec: &[u8] = &[0x00, 0x03, 0x41, 0x2F, 0x42, 0x00, 0x00];
        let (topic, packet_identifier, length) = get_variable_header(vec);
        assert_eq!(topic, "A/B".to_owned());
        assert_eq!(packet_identifier, [0x00, 0x00]);
        assert_eq!(length, 7);
    }
}
