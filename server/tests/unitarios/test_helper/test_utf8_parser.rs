#[cfg(test)]
mod tests {
    use server::helper::utf8_parser::UTF8;

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