pub struct UTF8 {}

impl UTF8 {
    fn check_minimum_length(bytes: &[u8]) -> Result<(), String> {
        if bytes.len() < 3 {
            return Err("Length should be at least 3".to_string());
        }
        Ok(())
    }

    fn check_malmormed_encoding(bytes: &[u8]) -> Result<(u8, u8), String> {
        let msb = bytes[0];
        let lsb = bytes[1];
        let encoding_length = bytes.len() - 2;
        if !(encoding_length >= (msb as usize) && encoding_length >= (lsb as usize)) {
            return Err("Malformed code".to_string());
        }
        Ok((msb, lsb))
    }

    pub fn utf8_parser(bytes: &[u8]) -> Result<(String, usize), String> {
        UTF8::check_minimum_length(bytes)?;
        let (msb, lsb) = UTF8::check_malmormed_encoding(bytes)?;

        let length: usize; // Length es el largo total del campo que estamos parseando: username, password etc
        let init: usize;
        let end: usize;
        let mut copy_bytes = bytes.to_vec();
        if msb > lsb {
            init = 0;
            end = usize::from(msb);
            copy_bytes = copy_bytes[2..end + 2].to_vec();
            copy_bytes.reverse();
            length = usize::from(msb + 0b00000010);
        } else {
            init = 2;
            end = usize::from(lsb) + 0b00000010;
            length = usize::from(lsb + 0b00000010);
        }

        Ok((
            String::from_utf8(copy_bytes[init..end].to_vec()).unwrap(),
            length,
        ))
    }
}
