pub struct UTF8 {}

impl UTF8 {
    pub fn utf8_parser(bytes: &[u8]) -> (String, usize) {
        let msb = bytes[0];
        let lsb = bytes[1];
        let mut copy = bytes.to_vec();
        let length: usize;
        let init: usize;
        let end: usize;

        if msb > lsb {
            copy.reverse();
            init = 0;
            end = usize::from(msb);
            length = end;
        } else {
            init = 2;
            end = 2 + usize::from(lsb);
            length = usize::from(lsb);
        }

        let string_result = String::from_utf8(copy[init..end].to_vec()).unwrap();
        (string_result, length)
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
        assert_eq!(length, 2);
    }

    #[test]
    fn parse_lsb() {
        let vec: &[u8] = &[0x00, 0x02, 0x41, 0x42, 0x23];
        let (string_result, length) = UTF8::utf8_parser(vec);
        assert_eq!(string_result, "AB".to_owned());
        assert_eq!(length, 2);
    }
}
