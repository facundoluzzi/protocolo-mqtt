use std::any::Any;

use crate::flags::flags::Flags;
use crate::payload::payload::Payload;
use crate::utf8_parser::UTF8;

pub struct ConnectPayload {
    client_identifier: String,
    will_topic: Option<String>,
    will_message: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl Payload for ConnectPayload {
    fn new(connect_flags: &Box<dyn Flags>, remaining_bytes: &[u8]) -> Box<dyn Payload> {
        let mut pointer: usize = 0;
        let client_identifier: String;
        let username: Option<String>;
        let password: Option<String>;
        let will_topic: Option<String>;
        let will_message: Option<String>;
        if remaining_bytes != &[0x00u8] {
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
            let (will_message_copy, index) =
                UTF8::utf8_parser(&remaining_bytes[pointer..remaining_bytes.len()]);
            will_message = Some(will_message_copy);
            pointer += index;
        } else {
            will_topic = None;
            will_message = None;
        }
        Box::new(ConnectPayload {
            client_identifier: client_identifier,
            username: username,
            password: password,
            will_topic: will_topic,
            will_message: will_message,
        })
    }
    fn get_client_id(&self) -> String {
        self.client_identifier.to_owned()
    }
    fn get_username(&self) -> Option<&String> {
        self.username.as_ref()
    }
    fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }
    fn get_will_topic(&self) -> Option<&String> {
        self.will_topic.as_ref()
    }
    fn get_will_message(&self) -> Option<&String> {
        self.will_message.as_ref()
    }
}


#[cfg(test)]
mod tests {
    use crate::flags::connect_flags::ConnectFlags;
    use super::*;

    #[test]
    fn create_payload_with_username() {
        let flags: u8 = 0b10000000;
        let connect_flags = ConnectFlags::new(&flags);
        let remaining_bytes = [
            0x00, 0x02, 0x5C, 0x0B, 0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F
        ]; 
        let connect = ConnectPayload::new(&connect_flags, &remaining_bytes);
        assert_eq!(connect.get_username(), Some("ALTEGO".to_owned()).as_ref()); 
        assert_eq!(connect.get_password(), None); 
        assert_eq!(connect.get_will_topic(), None);
        assert_eq!(connect.get_will_message(), None);
    }

    #[test]
    fn create_payload_with_username_and_password() {
        let flags: u8 = 0b11000000;
        let connect_flags = ConnectFlags::new(&flags);
        let remaining_bytes = [
            0x00, 0x02, 0x5C, 0x0B, 0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F,
            0x00, 0x03, 0x01, 0x02, 0x03
        ];
        let connect = ConnectPayload::new(&connect_flags, &remaining_bytes);
        println!("Mirar el formato del usuario: {}", connect.get_username().to_owned().unwrap()); 
        println!("Mirar el formato de la contraseña: {}", connect.get_password().to_owned().unwrap());
        assert_eq!(connect.get_username(), Some("ALTEGO".to_owned()).as_ref()); 
        assert_eq!(connect.get_password(), Some(String::from_utf8([1, 2, 3].to_vec()).unwrap()).as_ref()); 
        assert_eq!(connect.get_will_topic(), None);
        assert_eq!(connect.get_will_message(), None);
    }

    #[test]
    fn create_payload_with_will_topic_and_message() {
        let flags: u8 = 0b00111110;
        let connect_flags = ConnectFlags::new(&flags);
        let remaining_bytes = [
            0x00, 0x02, 0x5C, 0x0B, 
            0x00, 0x05, 0x54, 0x4F, 0x50, 0x49, 0x43,
            0x07, 0x00, 0x45, 0x47, 0x41, 0x53, 0x53, 0x45, 0x4D // EGASSEM en hexa, al parsearlo queda como MESSAGE
        ];
        let connect = ConnectPayload::new(&connect_flags, &remaining_bytes);
        assert_eq!(connect.get_username(), None); 
        assert_eq!(connect.get_password(), None); 
        assert_eq!(connect.get_will_topic(), Some("TOPIC".to_owned()).as_ref());
        assert_eq!(connect.get_will_message(), Some("MESSAGE".to_owned()).as_ref());
    }

    #[test]
    fn create_complete_payload() {
        let flags: u8 = 0b11111110;
        let connect_flags = ConnectFlags::new(&flags);
        let remaining_bytes = [
            0x00, 0x02, 0x5C, 0x0B, 0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F,
            0x00, 0x03, 0x01, 0x02, 0x03, 
            0x00, 0x05, 0x54, 0x4F, 0x50, 0x49, 0x43,
            0x07, 0x00, 0x45, 0x47, 0x41, 0x53, 0x53, 0x45, 0x4D // EGASSEM en hexa, al parsearlo queda como MESSAGE
        ];
        let connect = ConnectPayload::new(&connect_flags, &remaining_bytes);
        assert_eq!(connect.get_username(), Some("ALTEGO".to_owned()).as_ref()); 
        assert_eq!(connect.get_password(), Some(String::from_utf8([1, 2, 3].to_vec()).unwrap()).as_ref()); 
        assert_eq!(connect.get_will_topic(), Some("TOPIC".to_owned()).as_ref());
        assert_eq!(connect.get_will_message(), Some("MESSAGE".to_owned()).as_ref());
    }
}
