#[cfg(test)]
mod tests {
    use server::helper::remaining_length::save_remaining_length;

    #[test]
    fn obtener_remaining_length_12() {
        let bytes = [0x0C];
        let (readed_bytes, rem_length) = save_remaining_length(&bytes).unwrap();
        assert_eq!(readed_bytes, 1);
        assert_eq!(rem_length, 12);
    }

    #[test]
    fn obtener_remainin_length_127() {
        let bytes = [0x7F];
        let (readed_bytes, rem_length) = save_remaining_length(&bytes).unwrap();
        assert_eq!(readed_bytes, 1);
        assert_eq!(rem_length, 127);
    }

    #[test]
    fn obtener_remainin_length_128() {
        let bytes = [0x80, 0x01];
        let (readed_bytes, rem_length) = save_remaining_length(&bytes).unwrap();
        assert_eq!(readed_bytes, 2);
        assert_eq!(rem_length, 128);
    }

    #[test]
    fn obtener_remaining_length_129() {
        let bytes = [0x81, 0x01];
        let (readed_bytes, rem_length) = save_remaining_length(&bytes).unwrap();
        assert_eq!(readed_bytes, 2);
        assert_eq!(rem_length, 129);
    }

    #[test]
    fn obtener_remainin_length_129() {
        let bytes = [0xC1, 0x02];
        let (readed_bytes, rem_length) = save_remaining_length(&bytes).unwrap();
        assert_eq!(readed_bytes, 2);
        assert_eq!(rem_length, 321);
    }
}
