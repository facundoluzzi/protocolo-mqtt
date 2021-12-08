use crate::helper::remaining_length::save_remaining_length;
use crate::helper::utf8_parser::UTF8;
use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use crate::topics::subscriber::Subscriber;
use crate::topics::topic_types::TypeTopicManager;
use crate::usermanager::user_manager_action::UserManagerAction;
use crate::variable_header::subscribe_variable_header::get_variable_header;
use crate::wildcard::verify_wildcard;
use crate::wildcard::wildcard_result::WildcardResult::{
    HasNoWildcard, HasWildcard, InvalidWildcard,
};

use std::convert::TryInto;
use std::sync::mpsc::Sender;

pub struct Subscribe {
    remaining_length: usize,
    packet_identifier: [u8; 2],
    payload: Vec<u8>,
    return_codes: Vec<u8>,
}

impl Subscribe {
    pub fn init(bytes: &[u8]) -> Result<Subscribe, String> {
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len)?;

        let init_variable_header = 1 + readed_index;

        let variable_header = get_variable_header(&bytes[init_variable_header..bytes.len()]);
        let (_packet_identifier, length) = variable_header;

        let payload = &bytes[init_variable_header + length..bytes.len()];

        let packet_identifier = _packet_identifier[0..2]
            .try_into()
            .expect("slice with incorrect length");

        let subscribe = Subscribe {
            remaining_length,
            packet_identifier,
            payload: (*payload).to_vec(),
            return_codes: Vec::new(),
        };

        Ok(subscribe)
    }

    pub fn subscribe_topic(
        &mut self,
        sender_topic_manager: Sender<TypeTopicManager>,
        sender_user_manager: Sender<UserManagerAction>,
        client_id: String,
    ) -> Result<Self, String> {
        let mut acumulator: usize = 0;

        while self.payload.len() > acumulator {
            let (topic, length) =
                match UTF8::utf8_parser(&self.payload[acumulator..self.payload.len()]) {
                    Ok((topic, length)) => (topic, length),
                    Err(_err_result) => {
                        self.return_codes.push(0x80);
                        continue;
                    }
                };

            let wildcard = match verify_wildcard::get_wilcard(topic.to_owned()) {
                HasWildcard(wildcard) => Some(wildcard),
                HasNoWildcard => None,
                InvalidWildcard => {
                    acumulator += length + 1;
                    self.return_codes.push(0x80);
                    println!("a");
                    continue;
                }
            };

            let qos = self.payload[length + acumulator];

            let subscriber = Subscriber::init(
                client_id.to_string(),
                topic,
                sender_user_manager.clone(),
                wildcard,
                qos,
            );

            match sender_topic_manager.send(TypeTopicManager::Subscriber(subscriber)) {
                Ok(_) => {}
                Err(_) => {
                    acumulator += length + 1;
                    self.return_codes.push(0x80);
                    continue;
                }
            }

            acumulator += length + 1;

            match qos {
                0 => self.return_codes.push(0x00),
                1 => self.return_codes.push(0x01),
                _ => self.return_codes.push(0x80),
            };
        }

        let subscribe = Subscribe {
            remaining_length: self.remaining_length,
            packet_identifier: self.packet_identifier,
            payload: self.payload.clone(),
            return_codes: self.return_codes.clone(),
        };

        Ok(subscribe)
    }

    pub fn send_response(&self, sender_stream: Sender<StreamType>) {
        let packet_type = 0x90;
        let remaining_length = 0x03;
        let packet_identifier_msb = self.packet_identifier[0];
        let packet_identifier_lsb = self.packet_identifier[1];
        let mut bytes_response = vec![
            packet_type,
            remaining_length,
            packet_identifier_msb,
            packet_identifier_lsb,
        ];

        for return_code in &self.return_codes {
            bytes_response.push(*return_code);
        }

        if let Err(msg_error) =
            sender_stream.send((WriteStream, Some(bytes_response.to_vec()), None, None))
        {
            println!("Error in sending response: {}", msg_error);
        }
    }
}
