use crate::helper::publisher_subscriber_code::PublisherSubscriberCode;
use crate::helper::remaining_length::save_remaining_length;
use crate::helper::utf8_parser::UTF8;
use std::sync::mpsc::Sender;

use std::io::Write;
use std::net::TcpStream;

use super::publisher_suscriber::PublisherSuscriber;

pub struct Subscribe {
    _remaining_length: usize,
    _packet_identifier: u8,
    payload: Vec<u8>,
}

impl Subscribe {
    pub fn init(bytes: &[u8]) -> Subscribe {
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len).unwrap();

        let init_variable_header = 1 + readed_index;
        let end_variable_header = init_variable_header + 2;

        let payload = &bytes[end_variable_header..bytes.len()];

        Subscribe {
            _remaining_length: remaining_length,
            _packet_identifier: bytes[init_variable_header],
            payload: (*payload).to_vec(),
        }
    }

    pub fn get_type(&self) -> String {
        "subscribe".to_owned()
    }

    pub fn subscribe_topic(
        &self,
        sender: &Sender<PublisherSuscriber>,
        sender_for_publish: Sender<String>,
        client_id: String,
    ) -> Self {
        let mut acumulator: i32 = -1;

        while self.payload.len() as i32 > acumulator + 1 {
            let (topic, length) =
                UTF8::utf8_parser(&self.payload[(acumulator + 1) as usize..self.payload.len()]);
            acumulator += length as i32;
            let type_s = PublisherSubscriberCode::Subscriber;
            let message = "None".to_owned();
            let publisher_suscriber = PublisherSuscriber::new(
                topic,
                message,
                type_s,
                Some(sender_for_publish.clone()),
                client_id.to_string(),
            );
            if let Err(sender_err) = sender.send(publisher_suscriber) {
                println!("Error sending to publisher_subscriber: {}", sender_err);
            }
        }

        Subscribe {
            _remaining_length: self._remaining_length,
            _packet_identifier: self._packet_identifier,
            payload: self.payload.clone(),
        }
    }

    pub fn send_response(&self, mut stream: &TcpStream) {
        let bytes_response = [0x90, 0x03, 0x00, 0x00, 0x00];
        if let Err(msg_error) = stream.write(&bytes_response) {
            println!("Error in sending response: {}", msg_error);
        }
    }
}
