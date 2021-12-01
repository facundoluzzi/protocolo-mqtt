use crate::helper::publisher_subscriber_code::PublisherSubscriberCode;
use crate::helper::remaining_length::{save_remaining_length, self};
use crate::helper::utf8_parser::UTF8;
use crate::variable_header::subscribe_variable_header::get_variable_header;

use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

use super::publisher_suscriber::PublisherSuscriber;
pub struct Subscribe {
    remaining_length: usize,
    packet_identifier: Vec<u8>,
    payload: Vec<u8>,
    return_codes: Vec<u8>,
}

impl Subscribe {
    pub fn init(bytes: &[u8]) -> Subscribe {
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len).unwrap();

        let init_variable_header = 1 + readed_index;
        let packet_identifier: &[u8];
        packet_identifier = &[0x00u8,0x00u8];
        let variable_header = get_variable_header(&bytes[init_variable_header..bytes.len()]);
        if let Ok((packet_identifier_rec, length)) = variable_header{
            let payload = &bytes[init_variable_header + length..bytes.len()];
            let packet_identifier = packet_identifier_rec;
            return Subscribe {
                remaining_length,
                packet_identifier: packet_identifier,
                payload: (*payload).to_vec(),
                return_codes: Vec::new(),
            }
        }
        let payload = &bytes[init_variable_header + 2..bytes.len()];
        Subscribe {
            remaining_length,
            packet_identifier: packet_identifier.to_vec(),
            payload: (*payload).to_vec(),
            return_codes: Vec::new(),
        }
        }       
    

    pub fn subscribe_topic(
        &mut self,
        sender: &Sender<PublisherSuscriber>,
        sender_for_publish: Sender<String>,
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

            let qos = self.payload[length + acumulator];
            acumulator += length + 1;
            
            let type_s = PublisherSubscriberCode::Subscriber;
            let message = "None".to_owned();
            let publisher_subscriber = PublisherSuscriber::new(
                topic,
                message,
                type_s,
                Some(sender_for_publish.clone()),
                client_id.to_string(),
            );

            if let Err(sender_err) = sender.send(publisher_subscriber) {
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
            packet_identifier: self.packet_identifier.clone(),
            payload: self.payload.clone(),
            return_codes: self.return_codes.clone(),
        }
    }

    pub fn send_response(&self, mut stream: &TcpStream) {

        let packet_type = 0x90;
        let mut bytes_response = Vec::new();
        let remaining_length = self.packet_identifier.len()+3;

        bytes_response.push(packet_type);
        bytes_response.push(remaining_length as u8);
        bytes_response.push(0x00);
        bytes_response.push(self.packet_identifier.len() as u8);
        bytes_response = [bytes_response, self.packet_identifier.clone()].concat();


        for return_code in &self.return_codes {
            bytes_response.push(*return_code);
        }

        if let Err(msg_error) = stream.write(&bytes_response) {
            println!("Error in sending response: {}", msg_error);
        }
    }
}