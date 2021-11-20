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
    ) -> Self {
        let mut acumulator: i32 = -1;

        while self.payload.len() as i32 > acumulator + 1 {
            let (topic, length) =
                UTF8::utf8_parser(&self.payload[(acumulator + 1) as usize..self.payload.len()]);
            acumulator += length as i32;
            let type_s = PublisherSubscriberCode::Subscriber;
            let message = "None".to_owned();
            let publisher_suscriber =
                PublisherSuscriber::new(topic, message, type_s, Some(sender_for_publish.clone()));
            sender.send(publisher_suscriber).unwrap();
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

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::{thread, time};

    // #[test]
    // fn crear_paquete_subscribe_correctamente() {
    //     // El primer byte está compuesto por el control packet header y por algunos flags.
    //     // El segundo byte es el remaining length que indica el largo del variable header mas el payload
    //     // el tercer y cuarto byte indican el largo del topic name, según el encoding msb o lsb, en este caso de 5 bytes
    //     // el byte nro 10 y 11 son los dos últimos bytes del variable header y son el packet identifier
    //     // los n siguientes bugs son parte del payload, en este caso vacío
    //     let bytes = [0x30, 0x08, 0x00, 0x0A, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45];
    //     let sender = TopicManager::new();
    //     let publish_packet = Subscribe::init(&bytes).subscribe_topic(&sender.get_sender());
    //     Subscribe::init(&bytes).subscribe_topic(&sender.get_sender());
    //     Subscribe::init(&bytes).subscribe_topic(&sender.get_sender());
    //     thread::sleep(time::Duration::from_millis(100));
    //     assert_eq!(publish_packet.get_type(), "subscribe".to_owned());
    // }
}
