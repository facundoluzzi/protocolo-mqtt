use crate::helper::remaining_length::save_remaining_length;
use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use crate::topics::publisher::Publisher;
use crate::topics::topic_types::TypeTopicManager;
use crate::variable_header::publish_variable_header::{self, get_variable_header};

use std::convert::TryInto;
use std::sync::mpsc::Sender;

pub struct Publish {
    _dup: u8,
    qos: u8,
    _retain: u8,
    remaining_length: usize,
    topic: String,
    packet_identifier: [u8; 2],
    payload: String,
    all_bytes: Vec<u8>,
}

impl Publish {
    pub fn init(bytes: &[u8]) -> Publish {
        let mut dup_flag = (0x08 & bytes[0]) >> 3;
        let qos_flag = (0x06 & bytes[0]) >> 1;

        if qos_flag == 0x00 {
            dup_flag = 0x00;
        } else if qos_flag >= 0x02 {
            // TODO: finalizar conexión
        }

        let retain_flag = 0x01 & bytes[0];

        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len).unwrap();

        let init_variable_header = 1 + readed_index;

        let (topic, packet_identifier, length) =
            get_variable_header(&bytes[init_variable_header..bytes.len()]).unwrap();

        let _valid_topic = publish_variable_header::verify_publish_wilcard(topic.to_owned());

        // TODO: cerrar la conexión
        let payload = &bytes[init_variable_header + length..bytes.len()];

        Publish {
            _dup: dup_flag,
            _retain: retain_flag,
            qos: qos_flag,
            remaining_length,
            topic,
            packet_identifier: packet_identifier[0..2]
                .try_into()
                .expect("slice with incorrect length"),
            payload: std::str::from_utf8(payload).unwrap().to_string(),
            all_bytes: bytes.to_vec(),
        }
    }

    pub fn get_topic(&self) -> String {
        self.topic.to_string()
    }

    pub fn send_response(&self, stream: Sender<StreamType>) {
        match self.qos {
            0x00 => {}
            0x01 => {
                let puback_response = [
                    0x40,
                    0x02,
                    self.packet_identifier[0],
                    self.packet_identifier[1],
                ];
                if let Err(msg_error) =
                    stream.send((WriteStream, Some(puback_response.to_vec()), None, None))
                {
                    println!("Error in sending response: {}", msg_error);
                }
            }
            _ => {
                println!("Error");
            }
        }
    }

    pub fn send_message(
        &self,
        sender_topic_manager: &Sender<TypeTopicManager>,
        client_id: String,
    ) -> Self {
        let topic = self.topic.to_owned();

        let publisher_prueba = Publisher::init(client_id, topic, self.all_bytes.clone(), self.qos);

        if let Err(sender_err) =
            sender_topic_manager.send(TypeTopicManager::Publisher(publisher_prueba))
        {
            println!("Error sending to publisher_subscriber: {}", sender_err);
        }

        Publish {
            _dup: self._dup,
            qos: self.qos,
            _retain: self._retain,
            remaining_length: self.remaining_length,
            topic: self.topic.clone(),
            packet_identifier: self.packet_identifier,
            payload: self.payload.clone(),
            all_bytes: self.all_bytes.clone(),
        }
    }
}
