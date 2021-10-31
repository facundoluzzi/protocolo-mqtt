use crate::flags::flags::Flags;
use crate::payload::payload::Payload;

use crate::helper::utf8_parser::UTF8;

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

        if connect_flags.get_will_flag() {
            let (will_topic_copy, index) =
                UTF8::utf8_parser(&remaining_bytes[pointer + 1..remaining_bytes.len()]);
            will_topic = Some(will_topic_copy);
            pointer += index;
            let (will_message_copy, index) =
                UTF8::utf8_parser(&remaining_bytes[pointer + 1..remaining_bytes.len()]);
            will_message = Some(will_message_copy);
            pointer += index;
        } else {
            will_topic = None;
            will_message = None;
        }

        if connect_flags.get_username_flag() {
            let (username_copy, index) =
                UTF8::utf8_parser(&remaining_bytes[pointer + 1..remaining_bytes.len()]);
            username = Some(username_copy);
            pointer += index;
        } else {
            username = None;
        }

        if connect_flags.get_password_flag() & connect_flags.get_username_flag() {
            let (password_copy, index) =
                UTF8::utf8_parser(&remaining_bytes[pointer + 1..remaining_bytes.len()]);
            password = Some(password_copy);
            pointer += index;
        } else {
            password = None;
        }

        Box::new(ConnectPayload {
            client_identifier: client_identifier,
            username: username,
            password: password,
            will_topic: will_topic,
            will_message: will_message,
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn obtener_remaining_length_correctamente() {
//         let first_bytes = [
//             0x10, 0x0C, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x04, 0x00, 0x00, 0x0B, 0x01, 0x02,
//         ];

//         let first_connect_packet = ConnectPayload::new();
//         assert_eq!(first_connect_packet.get_remaining_length(), 12);

//      }
// }
