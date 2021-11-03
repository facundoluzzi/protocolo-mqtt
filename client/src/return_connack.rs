pub fn get_code(bytes: u8) -> String {
    match bytes {
        0x00 => "Connection Accepted".to_owned(),
        0x01 => "Connection Refused, unacceptable protocol version".to_owned(),
        0x02 => "Connection Refused, identifier rejected".to_owned(),
        0x03 => "Connection Refused, Server unavailable".to_owned(),
        0x04 => "Connection Refused, bad user name or password".to_owned(),
        0x05 => "Connection Refused, not authorized".to_owned(),
        _ => "Reserved".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcular_return_connack() {
        let prueba = get_code(0x00);
        assert_eq!(prueba, "Connection Accepted".to_owned());
    }
}
