pub struct ReturnConnack {
    return_code: String,
}

impl ReturnConnack {
    fn init(bytes: &[u8]) -> ReturnConnack{
        return_code = match bytes {
            0x00 => "Connection Accepted".to_owned(),
            0x01 => "Connection Refused, unacceptable protocol version".to_owned(),
            0x02 => "Connection Refused, identifier rejected".to_owned(),
            0x03 => "Connection Refused, Server unavailable".to_owned(), 
            0x04 => "Connection Refused, bad user name or password".to_owned(),
            0x05 => "Connection Refused, not authorized".to_owned(),
            _ => "Reserved".to_owned(),
        }
    }
    pub fn get_code(&self) -> String {
        self.return_code
    }
}