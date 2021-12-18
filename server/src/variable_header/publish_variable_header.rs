use crate::helper::utf8_parser::UTF8;

/// 
pub fn get_variable_header(
    bytes: &[u8],
    qos: u8,
) -> Result<(String, Option<&[u8]>, usize), String> {
    match UTF8::utf8_parser(bytes) {
        Ok((parsed_topic, readed_bytes)) => {
            let (packet_identifier, length) = if qos == 1 {
                (
                    Some(&bytes[readed_bytes..readed_bytes + 2]),
                    readed_bytes + 2,
                )
            } else {
                (None, 0)
            };

            Ok((parsed_topic, packet_identifier, length))
        }
        Err(err) => Err(err),
    }
}

pub fn verify_publish_wilcard(topic: String) -> bool {
    let chars: Vec<char> = topic.chars().collect();

    let first_char = match chars.get(0) {
        Some(char_found) => char_found,
        None => panic!("Unexpected error"),
    };

    let last_char = match topic.chars().last() {
        Some(char_found) => char_found,
        None => panic!("Unexpected error"),
    };

    let contains_wilcards = topic.contains('#') || topic.contains('$') || topic.contains('+');

    (!contains_wilcards) && (last_char != '/' && first_char != &'/')
}
