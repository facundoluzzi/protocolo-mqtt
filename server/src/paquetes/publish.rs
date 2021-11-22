use crate::helper::publisher_subscriber_code::PublisherSubscriberCode::Publisher;
use crate::helper::remaining_length::save_remaining_length;
use crate::variable_header::publish_variable_header::get_variable_header;

use std::convert::TryInto;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

use super::publisher_suscriber::PublisherSuscriber;

pub struct Publish {
    _dup: u8,
    qos: u8,
    _retain: u8,
    _remaining_length: usize,
    _topic: String,
    packet_identifier: [u8; 2],
    _payload: String,
}

impl Publish {
    pub fn init(bytes: &[u8]) -> Publish {
        let dup_flag = (0x08 & bytes[0]) >> 3;
        let qos_flag = (0x06 & bytes[0]) >> 1;
        let retain_flag = 0x01 & bytes[0];

        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len).unwrap();

        let init_variable_header = 1 + readed_index;

        let (topic, packet_identifier, length) =
            get_variable_header(&bytes[init_variable_header..bytes.len()]);
        let payload = &bytes[init_variable_header + length..bytes.len()];

        Publish {
            _dup: dup_flag,
            qos: qos_flag,
            _retain: retain_flag,
            _remaining_length: remaining_length,
            _topic: topic,
            packet_identifier: packet_identifier[0..2]
                .try_into()
                .expect("slice with incorrect length"),
            _payload: std::str::from_utf8(payload).unwrap().to_string(),
        }
    }

    pub fn get_topic(&self) -> String {
        self._topic.to_string()
    }

    pub fn send_response(&self, mut stream: &TcpStream) {
        match self.qos {
            0x00 => {}
            0x01 => {
                let puback_response = [
                    0x40,
                    0x01,
                    self.packet_identifier[0],
                    self.packet_identifier[1],
                ];
                if let Err(msg_error) = stream.write(&puback_response) {
                    println!("Error in sending response: {}", msg_error);
                }
            }
            _ => {
                println!("Error");
            }
        }
    }

    pub fn send_message(&self, sender: &Sender<PublisherSuscriber>, client_id: String) -> Self {
        let topic = self._topic.to_owned();
        let payload = self._payload.to_owned();
        let publisher_suscriber =
            PublisherSuscriber::new(topic, payload, Publisher, None, client_id);
        sender.send(publisher_suscriber).unwrap();
        Publish {
            _dup: self._dup,
            qos: self.qos,
            _retain: self._retain,
            _remaining_length: self._remaining_length,
            _topic: self._topic.clone(),
            packet_identifier: self.packet_identifier,
            _payload: self._payload.clone(),
        }
    }
}
