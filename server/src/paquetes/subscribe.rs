use crate::helper::publisher_subscriber_code::PublisherSubscriberCode;
use crate::helper::remaining_length::save_remaining_length;
use crate::helper::utf8_parser::UTF8;
use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use crate::usermanager::user_manager_action::UserManagerAction::GetSenderUserManager;
use crate::usermanager::user_manager_types::ChannelUserManager;
use crate::variable_header::subscribe_variable_header::get_variable_header;

use std::convert::TryInto;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use super::publisher_suscriber::PublisherSuscriber;

pub struct Subscribe {
    remaining_length: usize,
    packet_identifier: [u8; 2],
    payload: Vec<u8>,
    return_codes: Vec<u8>,
}

impl Subscribe {
    // TODO: verificar el largo del payload mayor a 1
    pub fn init(bytes: &[u8]) -> Subscribe {
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len).unwrap();

        let init_variable_header = 1 + readed_index;

        let variable_header = get_variable_header(&bytes[init_variable_header..bytes.len()]);
        let (_packet_identifier, length) = variable_header;

        let payload = &bytes[init_variable_header + length..bytes.len()];

        let packet_identifier = _packet_identifier[0..2]
            .try_into()
            .expect("slice with incorrect length");

        Subscribe {
            remaining_length,
            packet_identifier,
            payload: (*payload).to_vec(),
            return_codes: Vec::new(),
        }
    }

    pub fn subscribe_topic(
        &mut self,
        sender_topic_manager: Sender<PublisherSuscriber>,
        sender_user_manager: Sender<ChannelUserManager>,
        client_id: String,
    ) -> Self {
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

            let (sender, receiver): (Sender<Sender<String>>, Receiver<Sender<String>>) =
                mpsc::channel();

            sender_user_manager
                .send((
                    GetSenderUserManager,
                    client_id.to_owned(),
                    None,
                    None,
                    Some(sender),
                ))
                .unwrap();

            let sender_received = receiver.recv().unwrap();

            let qos = self.payload[length + acumulator];
            acumulator += length + 1;

            let type_s = PublisherSubscriberCode::Subscriber;
            let message = "None".to_owned();
            let publisher_subscriber = PublisherSuscriber::new(
                topic,
                message,
                type_s,
                Some(sender_received),
                client_id.to_string(),
            );

            if let Err(sender_err) = sender_topic_manager.send(publisher_subscriber) {
                println!("Error sending to publisher_subscriber: {}", sender_err);
            }

            match qos {
                0 => self.return_codes.push(0x00),
                1 => self.return_codes.push(0x01),
                _ => self.return_codes.push(0x80),
            };
        }

        Subscribe {
            remaining_length: self.remaining_length,
            packet_identifier: self.packet_identifier,
            payload: self.payload.clone(),
            return_codes: self.return_codes.clone(),
        }
    }

    pub fn send_response(&self, stream: Sender<StreamType>) {
        let packet_type = 0x90;
        let remaining_length = 0x03;
        let packet_identifier_msb = self.packet_identifier[0];
        let packet_identifier_lsb = self.packet_identifier[1];
        let mut bytes_response = Vec::new();

        bytes_response.push(packet_type);
        bytes_response.push(remaining_length);
        bytes_response.push(packet_identifier_msb);
        bytes_response.push(packet_identifier_lsb);

        for return_code in &self.return_codes {
            bytes_response.push(*return_code);
        }

        if let Err(msg_error) = stream.send((WriteStream, Some(bytes_response.to_vec()), None)) {
            println!("Error in sending response: {}", msg_error);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::variable_header::subscribe_variable_header;

    #[test]
    fn test_wilcard() {
        let prueba = subscribe_variable_header::verify_wilcard("A/B#".to_owned());
        let prueba2 = subscribe_variable_header::verify_wilcard("A/#".to_owned());

        assert_ne!(prueba, false);
        assert!(prueba2);
    }
}
