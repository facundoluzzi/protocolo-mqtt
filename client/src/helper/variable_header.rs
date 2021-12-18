use crate::helper::utf8_parser::UTF8;

pub fn get_variable_header(
    bytes: &[u8],
    qos: u8,
) -> Result<(String, Option<Vec<u8>>, usize), String> {
    match UTF8::utf8_parser(bytes) {
        Ok((parsed_topic, readed_bytes)) => {
            let (packet_identifier, length) = if qos == 1 {
                println!("LA CONCHA DE LA LORA");
                (
                    Some(bytes[readed_bytes..readed_bytes + 2].to_vec()),
                    readed_bytes + 4,
                )
            } else {
                (None, readed_bytes + 2)
            };

            Ok((parsed_topic, packet_identifier, length))
        }
        Err(err) => Err(err),
    }
}
