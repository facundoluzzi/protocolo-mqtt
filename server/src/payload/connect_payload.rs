use crate::flags::connect_flags::ConnectFlags;
use crate::helper::status_code::ConnectReturnCode;
use crate::helper::utf8_parser::UTF8;

#[derive(Debug)]
pub struct ConnectPayload {
    will_topic: Option<String>,
    will_message: Option<String>,
    client_identifier: String,
    username: Option<String>,
    password: Option<String>,
}

impl ConnectPayload {
    fn parse_client_id(bytes: &[u8]) -> Result<(String, usize), String> {
        let parser = UTF8::utf8_parser;
        if bytes != [0x00u8] {
            if let Ok((client_identifier_copy, index)) = parser(bytes) {
                Ok((client_identifier_copy, index))
            } else {
                Err("error parsing client identifier".to_string())
            }
        } else {
            Ok(("PayloadNull".to_owned(), 0))
        }
    }

    fn parse_last_will_topic_msg(
        bytes: &[u8],
        flags: &ConnectFlags,
        mut pointer: usize,
    ) -> Result<(Option<String>, Option<String>, usize), String> {
        let parser = UTF8::utf8_parser;
        if flags.get_will_flag() {
            let bytes_will_topic_to_parse = &bytes[pointer..bytes.len()];
            if let Ok((will_topic_copy, index)) = parser(bytes_will_topic_to_parse) {
                pointer += index;
                let bytes_will_message_to_parse = &bytes[pointer..bytes.len()];
                if let Ok((will_message_copy, index)) = parser(bytes_will_message_to_parse) {
                    Ok((
                        Some(will_topic_copy),
                        Some(will_message_copy),
                        pointer + index,
                    ))
                } else {
                    Err("Error parsing will message".to_string())
                }
            } else {
                Err("Error parsing will flag".to_string())
            }
        } else {
            Ok((None, None, pointer))
        }
    }

    fn parse_username_password(
        bytes: &[u8],
        flags: &ConnectFlags,
        mut pointer: usize,
    ) -> Result<(Option<String>, Option<String>), String> {
        let parser = UTF8::utf8_parser;
        if flags.get_username_flag() {
            let username_to_validate = &bytes[pointer..bytes.len()];
            if let Ok((username_copy, index)) = parser(username_to_validate) {
                pointer += index;
                if !flags.get_password_flag() {
                    Ok((Some(username_copy), None))
                } else {
                    let password_to_validate = &bytes[pointer..bytes.len()];
                    if let Ok((password_copy, _index)) = parser(password_to_validate) {
                        Ok((Some(username_copy), Some(password_copy)))
                    } else {
                        Err("Error parsing password".to_string())
                    }
                }
            } else {
                Err("Error parsing username".to_string())
            }
        } else {
            Ok((None, None))
        }
    }

    fn check_username_password(
        username: Option<String>,
        password: Option<String>,
        mut return_code: ConnectReturnCode,
    ) -> ConnectReturnCode {
        return_code = match (username.clone(), password.clone()) {
            (Some(uname), Some(pass)) => {
                return_code = return_code.check_malformed_username(uname);
                return_code.check_malformed_password(pass)
            }
            (Some(uname), None) => {
                return_code = return_code.check_malformed_username(uname);
                return_code.check_malformed_password("".to_string())
            }
            _ => return_code,
        };
        return_code.check_authentication(username, password)
    }

    pub fn init(
        flags: &ConnectFlags,
        remaining_bytes: &[u8],
        mut return_code: ConnectReturnCode,
    ) -> Result<(ConnectPayload, ConnectReturnCode), String> {
        let (client_identifier, pointer) = ConnectPayload::parse_client_id(remaining_bytes)?;
        let (will_topic, will_message, pointer) =
            ConnectPayload::parse_last_will_topic_msg(remaining_bytes, flags, pointer)?;
        let (username, password) =
            ConnectPayload::parse_username_password(remaining_bytes, flags, pointer)?;

        return_code = ConnectPayload::check_username_password(
            username.clone(),
            password.clone(),
            return_code,
        );
        let new_connect_payload = ConnectPayload {
            client_identifier,
            username,
            password,
            will_topic,
            will_message,
        };

        Ok((new_connect_payload, return_code))
    }

    pub fn get_username(&self) -> Option<&String> {
        self.username.as_ref()
    }

    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    pub fn get_client_id(&self) -> String {
        self.client_identifier.to_owned()
    }

    pub fn get_will_topic(&self) -> Option<String> {
        self.will_topic.to_owned()
    }

    pub fn get_will_message(&self) -> Option<String> {
        self.will_message.to_owned()
    }
}
