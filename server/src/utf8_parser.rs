pub struct UTF8 {}

impl UTF8 {
    pub fn utf8_parser(bytes: &[u8]) -> (String, usize) {
        let msb = bytes[0];
        let lsb = bytes[1];
        // Length es el largo total del campo que estamos parseando: username, password etc
        let length: usize;
        let init: usize;
        let end: usize;
        let mut copy_bytes = bytes.to_vec();
        if msb > lsb {
            copy_bytes.reverse();
            init = 0;
            end = usize::from(msb);
            length = usize::from(msb + 0b00000010);
        } else {
            init = 2;
            end = usize::from(lsb) + 0b00000010;
            length = usize::from(lsb + 0b00000010);
        }
        (
            String::from_utf8(copy_bytes[init..end].to_vec()).unwrap(),
            length,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_msb() {
        let vec: &[u8] = &[0x02, 0x00, 0x41, 0x42];
        let (string_result, length) = UTF8::utf8_parser(vec);
        assert_eq!(string_result, "BA".to_owned());
        assert_eq!(length, 4);
    }

    #[test]
    fn parse_lsb() {
        let vec: &[u8] = &[0x00, 0x02, 0x41, 0x42];
        let (string_result, length) = UTF8::utf8_parser(vec);
        assert_eq!(string_result, "AB".to_owned());
        assert_eq!(length, 4);
    }
}
