use crate::helper::utf8_parser::UTF8;
/**
 * https://docs.solace.com/PubSub-Basics/SMF-Topics.htm
 */
pub fn get_variable_header(bytes: &[u8]) -> (String, &[u8], usize) {
    let (parsed_topic, readed_bytes) = UTF8::utf8_parser(bytes); // topic 5 bytes, 7 bytes leidos
    let packet_identifier = &bytes[readed_bytes..readed_bytes + 2];
    (parsed_topic, packet_identifier, readed_bytes + 2)
}
