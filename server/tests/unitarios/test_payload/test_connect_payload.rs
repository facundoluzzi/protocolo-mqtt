#[cfg(test)]
mod tests {
    use server::{flags::connect_flags::ConnectFlags, helper::status_code::ConnectReturnCode, payload::connect_payload::ConnectPayload};

    #[test]
    fn create_payload_with_username() {
        let flags: u8 = 0xB0;
        let connect_flags = ConnectFlags::init(&flags);
        let remaining_bytes = [
            0x00, 0x02, 0x5C, 0x0B, 0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F,
        ];
        let connect_return_code = ConnectReturnCode::init();
        let (connect, return_code) =
            ConnectPayload::init(&connect_flags, &remaining_bytes, connect_return_code);
        assert_eq!(connect.get_will_topic(), None);
        assert_eq!(connect.get_will_message(), None);
        assert_eq!(connect.get_username(), Some(&"ALTEGO".to_string()));
        assert_eq!(connect.get_password(), None);
        assert_eq!(return_code.apply_validations(), 0x04);
    }

    #[test]
    fn create_payload_with_username_and_password() {
        let flags: u8 = 0xC0;
        let connect_flags = ConnectFlags::init(&flags);
        let remaining_bytes = [
            0x00, 0x02, 0x5C, 0x0B, 0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, 0x00, 0x03,
            0x41, 0x4C, 0x54,
        ];
        let connect_return_code = ConnectReturnCode::init();
        let (connect, return_code) =
            ConnectPayload::init(&connect_flags, &remaining_bytes, connect_return_code);
        assert_eq!(connect.get_password(), Some(&"ALT".to_string()));
        assert_eq!(connect.get_username(), Some(&"ALTEGO".to_string()));
        assert_eq!(connect.get_will_topic(), None);
        assert_eq!(connect.get_will_message(), None);
        assert_eq!(return_code.apply_validations(), 0x00);
    }

    #[test]
    fn create_payload_with_will_topic_and_message() {
        let flags: u8 = 0x3E;
        let connect_flags = ConnectFlags::init(&flags);
        let remaining_bytes = [
            0x00, 0x02, 0x5C, 0x0B, 0x00, 0x05, 0x54, 0x4F, 0x50, 0x49, 0x43, 0x07, 0x00, 0x45,
            0x47, 0x41, 0x53, 0x53, 0x45,
            0x4D, // EGASSEM en hexa, al parsearlo queda como MESSAGE
        ];
        let connect_return_code = ConnectReturnCode::init();
        let (connect, return_code) =
            ConnectPayload::init(&connect_flags, &remaining_bytes, connect_return_code);
        assert_eq!(connect.get_will_topic(), Some("TOPIC".to_owned()).as_ref());
        assert_eq!(
            connect.get_will_message(),
            Some("MESSAGE".to_owned()).as_ref()
        );
        assert_eq!(return_code.apply_validations(), 0x00);
    }

    #[test]
    fn create_complete_payload() {
        let flags: u8 = 0b11111110;
        let connect_flags = ConnectFlags::init(&flags);
        let remaining_bytes = [
            0x00, 0x02, 0x5C, 0x0B, 0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, 0x00, 0x03,
            0x01, 0x02, 0x03, 0x00, 0x05, 0x54, 0x4F, 0x50, 0x49, 0x43, 0x07, 0x00, 0x45, 0x47,
            0x41, 0x53, 0x53, 0x45, 0x4D, // EGASSEM en hexa, al parsearlo queda como MESSAGE
        ];
        let connect_return_code = ConnectReturnCode::init();
        let (connect, _) =
            ConnectPayload::init(&connect_flags, &remaining_bytes, connect_return_code);
        assert_eq!(connect.get_will_topic(), Some("TOPIC".to_owned()).as_ref());
        assert_eq!(
            connect.get_will_message(),
            Some("MESSAGE".to_owned()).as_ref()
        );
    }
}
