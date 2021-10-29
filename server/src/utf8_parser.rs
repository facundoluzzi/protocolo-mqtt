pub struct UTF8 {}

impl UTF8 {
    pub fn utf8_parser(bytes: &[u8]) -> (String, usize) {
        let msb = bytes[0];
        let lsb = bytes[1];
        let length: usize;
        if msb > lsb {
            length = usize::from(msb + 0b00000010);
        } else {
            length = usize::from(lsb + 0b00000010);
        }
        (
            String::from_utf8(bytes[2..length].to_vec()).unwrap(),
            length,
        )
    }
}