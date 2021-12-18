///Checkea payload de subscribe y unsubscribe para desconectar
pub fn check_payload(bytes: &[u8]) -> Result<&[u8], String> {
    if bytes.is_empty() {
        Err("Subscribe packet with no topics".to_string())
    } else {
        Ok(bytes)
    }
}
