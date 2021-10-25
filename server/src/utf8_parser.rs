pub struct UTF8 {}

impl UTF8 {
    pub fn utf8_parser(bytes: &[u8]) -> (&[u8], usize) {
        let msb = bytes[0];
        let lsb = bytes[1];
        let length: usize;

        if msb > lsb {
            length = usize::from(msb);
        } else {
            length = usize::from(lsb);
        }
        (&bytes[0..length], length)
    }
}
