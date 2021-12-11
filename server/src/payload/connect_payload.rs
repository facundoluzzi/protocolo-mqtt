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
    pub fn init(
        connect_flags: &ConnectFlags,
        remaining_bytes: &[u8],
        mut return_code: ConnectReturnCode,
    ) -> Result<(ConnectPayload, ConnectReturnCode), String> {
        let mut pointer: usize = 0;
        let client_identifier: String;
        let username: Option<String>;
        let password: Option<String>;
        let will_topic: Option<String>;
        let will_message: Option<String>;

        let parser = UTF8::utf8_parser;

        if remaining_bytes != [0x00u8] {
            if let Ok((client_identifier_copy, index)) = parser(remaining_bytes) {
                client_identifier = client_identifier_copy;
                pointer += index;
            } else {
                return Err("error parsing client identifier".to_string());
            }
        } else {
            client_identifier = "PayloadNull".to_owned();
        }

        if connect_flags.get_will_flag() {
            if let Ok((will_topic_copy, index)) =
                parser(&remaining_bytes[pointer..remaining_bytes.len()])
            {
                will_topic = Some(will_topic_copy);
                pointer += index;
                if let Ok((will_message_copy, index)) =
                    parser(&remaining_bytes[pointer..remaining_bytes.len()])
                {
                    will_message = Some(will_message_copy);
                    pointer += index;
                } else {
                    return Err("Error parsing will message".to_string());
                } 
            } else {
                return Err("Error parsing will flag".to_string());
            }
        } else {
            will_topic = None;
            will_message = None;
        }

        if connect_flags.get_username_flag() {
            let username_to_validate = &remaining_bytes[pointer..remaining_bytes.len()];
            if let Ok((username_copy, index)) = parser(username_to_validate) {
                return_code = return_code.check_malformed_username(username_copy.to_string());
                username = Some(username_copy);
                pointer += index;

                if !connect_flags.get_password_flag() {
                    return_code = return_code.check_malformed_password("".to_string());
                    password = None;
                } else {
                    let password_to_validate = &remaining_bytes[pointer..remaining_bytes.len()];
                    if let Ok((password_copy, _index)) = parser(password_to_validate) {
                        return_code =
                            return_code.check_malformed_password(password_copy.to_string());
                        password = Some(password_copy);
                    } else {
                        return Err("Error parsing password".to_string());
                    }
                }
            } else {
                return Err("Error parsing username".to_string());
            }
        } else {
            username = None;
            password = None;
        }

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
