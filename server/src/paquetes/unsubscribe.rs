use crate::helper::publisher_subscriber_code::PublisherSubscriberCode;
use crate::helper::remaining_length::{save_remaining_length};
use crate::helper::utf8_parser::UTF8;
use crate::variable_header::subscribe_variable_header::get_variable_header;

use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

use super::publisher_suscriber::PublisherSuscriber;

pub struct Unsubscribe {
    remaining_length: usize,
    packet_identifier: Vec<u8>,
    payload: Vec<u8>,
}

impl Unsubscribe {
    // TODO: verificar el largo del payload mayor a 1
    pub fn init(bytes: &[u8]) -> Unsubscribe {
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len).unwrap();

        let init_variable_header = 1 + readed_index;
        let packet_identifier: &[u8];
        packet_identifier = &[0x00u8,0x00u8];
        let variable_header = get_variable_header(&bytes[init_variable_header..bytes.len()]);
        if let Ok((packet_identifier_rec, length)) = variable_header{
            let payload = &bytes[init_variable_header + length..bytes.len()];
            let packet_identifier = packet_identifier_rec;
            return Unsubscribe {
                remaining_length,
                packet_identifier: packet_identifier,
                payload: (*payload).to_vec(),
            }
        }
        let payload = &bytes[init_variable_header + 2..bytes.len()];
        Unsubscribe {
            remaining_length,
            packet_identifier: packet_identifier.to_vec(),
            payload: (*payload).to_vec(),
        }
        }       
    

    pub fn unsubscribe_topic(
        &mut self,
        sender: &Sender<PublisherSuscriber>,
        client_id: String,
    ) -> Self {
        let mut acumulator: usize = 0;

        while self.payload.len() > acumulator {
            let (topic, length) =
                match UTF8::utf8_parser(&self.payload[acumulator..self.payload.len()]) {
                    Ok((topic, length)) => (topic, length),
                    Err(err_result) => {
                        println!("{}", err_result);
                        continue;
                    }
                };
            acumulator += length ;

            let type_s = PublisherSubscriberCode::Unsubscribe;
            let message = "None".to_owned();
            let publisher_subscriber = PublisherSuscriber::new(
                topic,
                message,
                type_s,
                None,
                client_id.to_string(),
            );

            if let Err(sender_err) = sender.send(publisher_subscriber) {
                println!("Error sending to publisher_subscriber: {}", sender_err);
            }

        }

        Unsubscribe {
            remaining_length: self.remaining_length,
            packet_identifier: self.packet_identifier.clone(),
            payload: self.payload.clone(),
        }
    }

    pub fn send_response(&self, mut stream: &TcpStream) {
        let packet_type = 0x90u8;
        let packet_identifier = self.packet_identifier.clone();
        let mut bytes_response = Vec::new();
        let remaining_length = packet_identifier.len()+3;

        bytes_response.push(packet_type);
        bytes_response.push(remaining_length as u8);
        bytes_response.push(0x00);
        bytes_response.push(packet_identifier.len() as u8);
        bytes_response = [bytes_response, packet_identifier].concat();


        if let Err(msg_error) = stream.write(&bytes_response) {
            println!("Error in sending response: {}", msg_error);
        }
    }
}