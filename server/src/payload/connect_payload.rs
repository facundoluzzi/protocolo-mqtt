use crate::flags::connect_flags::ConnectFlags;
use crate::helper::status_code::ConnectReturnCode;
use crate::helper::utf8_parser::UTF8;

pub struct ConnectPayload {
    _client_identifier: String,
    _will_topic: Option<String>,
    _will_message: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl ConnectPayload {
    pub fn init(
        connect_flags: &ConnectFlags,
        remaining_bytes: &[u8],
        mut return_code: ConnectReturnCode,
    ) -> (ConnectPayload, ConnectReturnCode) {
        let mut pointer: usize = 0;
        let client_identifier: String;
        let username: Option<String>;
        let password: Option<String>;
        let will_topic: Option<String>;
        let will_message: Option<String>;

        let parser = UTF8::utf8_parser;

        if remaining_bytes != [0x00u8] {
            let (client_identifier_copy, index) = parser(remaining_bytes);
            client_identifier = client_identifier_copy;
            pointer += index;
        } else {
            client_identifier = "PayloadNull".to_owned();
        }

        if connect_flags.get_username_flag() {
            let (username_copy, index) = parser(&remaining_bytes[pointer..remaining_bytes.len()]);
            return_code = return_code.check_malformed_username(username_copy.to_string());
            username = Some(username_copy);
            pointer += index;

            if !connect_flags.get_password_flag() {
                return_code = return_code.check_malformed_password("".to_string());
                password = None;
            } else {
                let (password_copy, index) =
                    parser(&remaining_bytes[pointer..remaining_bytes.len()]);
                return_code = return_code.check_malformed_password(password_copy.to_string());
                password = Some(password_copy);
                pointer += index;
            }
        } else {
            username = None;
            password = None;
        }

        if connect_flags.get_will_flag() {
            let (will_topic_copy, index) = parser(&remaining_bytes[pointer..remaining_bytes.len()]);
            will_topic = Some(will_topic_copy);
            pointer += index;
            let (will_message_copy, _index) =
                parser(&remaining_bytes[pointer..remaining_bytes.len()]);
            will_message = Some(will_message_copy);
        } else {
            will_topic = None;
            will_message = None;
        }
        let new_connect_payload = ConnectPayload {
            _client_identifier: client_identifier,
            username,
            password,
            _will_topic: will_topic,
            _will_message: will_message,
        };

        (new_connect_payload, return_code)
    }

    pub fn get_username(&self) -> Option<&String> {
        self.username.as_ref()
    }

    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    pub fn get_client_id(&self) -> String {
        self._client_identifier.to_owned()
    }

    pub fn get_will_topic(&self) -> Option<&String> {
        self._will_topic.as_ref()
    }

    pub fn get_will_message(&self) -> Option<&String> {
        self._will_message.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flags::connect_flags::ConnectFlags;

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
