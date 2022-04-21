use crate::helper::utf8_parser::UTF8;

/// Obtiene la longitud y el packet ID que pertenecen al variable header de un paquete
pub fn get_variable_header(
    bytes: &[u8],
    qos: u8,
) -> Result<(String, Option<Vec<u8>>, usize), String> {
    match UTF8::utf8_parser(bytes) {
        Ok((parsed_topic, readed_bytes)) => {
            let (packet_identifier, length) = if qos == 1 {
                (
                    Some(bytes[readed_bytes..readed_bytes + 2].to_vec()),
                    readed_bytes + 2,
                )
            } else {
                (None, readed_bytes)
            };

            Ok((parsed_topic, packet_identifier, length))
        }
        Err(err) => Err(err),
    }
}
