use crate::helper::utf8_parser::UTF8;
/**
 * https://docs.solace.com/PubSub-Basics/SMF-Topics.htm
 */
pub fn get_variable_header(bytes: &[u8]) -> Result<(String, &[u8], usize), String> {
    match UTF8::utf8_parser(bytes) {
        Ok((parsed_topic, readed_bytes)) => {
            let packet_identifier = &bytes[readed_bytes..readed_bytes + 2];
            Ok((parsed_topic, packet_identifier, readed_bytes + 2))
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn verify_publish_wilcard(topic: String) -> bool {
    let last_index = topic.len() - 1;
    return !(topic.contains("#") || topic.contains("$") || topic.contains("+")) && topic.chars().last().unwrap() != '/';
}