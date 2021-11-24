use crate::authentication::main::is_authenticated;
use crate::flags::connect_flags::ConnectFlags;
use crate::helper::file_handler::get_lines_as_key_values;
use crate::helper::utf8_parser::UTF8;

#[derive(Debug)]
pub struct ConnectPayload {
    _client_identifier: String,
    _will_topic: Option<String>,
    _will_message: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl ConnectPayload {
    pub fn init(connect_flags: &ConnectFlags, remaining_bytes: &[u8]) -> ConnectPayload {
        let mut pointer: usize = 0;
        let client_identifier: String;
        let username: Option<String>;
        let password: Option<String>;
        let will_topic: Option<String>;
        let will_message: Option<String>;

        if remaining_bytes != [0x00u8] {
            let (client_identifier_copy, index) = UTF8::utf8_parser(remaining_bytes);
            client_identifier = client_identifier_copy;
            pointer += index;
        } else {
            client_identifier = "PayloadNull".to_owned();
        }

        if connect_flags.get_username_flag() {
            let (username_copy, index) =
                UTF8::utf8_parser(&remaining_bytes[pointer..remaining_bytes.len()]);
            username = Some(username_copy);
            pointer += index;
        } else {
            username = None;
        }

        if connect_flags.get_password_flag() & connect_flags.get_username_flag() {
            let (password_copy, index) =
                UTF8::utf8_parser(&remaining_bytes[pointer..remaining_bytes.len()]);
            password = Some(password_copy);
            pointer += index;
        } else {
            password = None;
        }

        if connect_flags.get_will_flag() {
            let (will_topic_copy, index) =
                UTF8::utf8_parser(&remaining_bytes[pointer..remaining_bytes.len()]);
            will_topic = Some(will_topic_copy);
            pointer += index;
            let (will_message_copy, _index) =
                UTF8::utf8_parser(&remaining_bytes[pointer..remaining_bytes.len()]);
            will_message = Some(will_message_copy);
        } else {
            will_topic = None;
            will_message = None;
        }
        ConnectPayload {
            _client_identifier: client_identifier,
            username,
            password,
            _will_topic: will_topic,
            _will_message: will_message,
        }
    }

    pub fn check_authentication(&self, path: String) -> bool {
        match (self.username.as_ref(), self.password.as_ref()) {
            (Some(uname), Some(pass)) => {
                let credentials = get_lines_as_key_values(path);
                is_authenticated(uname.to_string(), pass.to_string(), credentials)
            }
            (None, None) => true,
            _ => false,
        }
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
