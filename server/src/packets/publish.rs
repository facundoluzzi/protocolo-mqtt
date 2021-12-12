use crate::packets::packet_manager::PacketManager;
use crate::enums::topic_manager::publisher::Publisher;
use crate::enums::topic_manager::topic_message::TypeMessage;
use crate::helper::remaining_length::save_remaining_length;
use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use crate::variable_header::publish_variable_header::{get_variable_header};

use std::sync::mpsc::Sender;

pub struct Publish {
    _dup: u8,
    qos: u8,
    retain: u8,
    remaining_length: usize,
    topic: String,
    packet_identifier: [u8; 2],
    payload: String,
    all_bytes: Vec<u8>,
}

impl Publish {

    pub fn process_message(bytes: &[u8], packet_manager: &mut PacketManager) -> Result<(), String> {
        if packet_manager.is_disconnected() {
            Err("Client is not connected".to_string())
        } else {
            let sender_stream = packet_manager.get_sender_stream();
            let sender_topic_manager = packet_manager.get_sender_topic_manager();

            let mut publish = Publish::init(bytes)?;
            publish = publish.send_message(sender_topic_manager, packet_manager.get_client_id())?;
            publish.send_response(sender_stream)?;
            Ok(())
        }
    }

    fn get_flags(bytes: &[u8]) -> Result<(u8, u8, u8), String> {
        let mut dup_flag = (0x08 & bytes[0]) >> 3;
        let qos_flag = (0x06 & bytes[0]) >> 1;
        let retain_flag = 0x01 & bytes[0];

        if qos_flag == 0x00 {
            dup_flag = 0x00;
            Ok((qos_flag, dup_flag, retain_flag))
        } else if qos_flag >= 0x02 {
            Err("qos can not be 2".to_string())
        } else {
            Ok((qos_flag, dup_flag, retain_flag))
        }
    }

    pub fn init(bytes: &[u8]) -> Result<Publish, String> {
        let (qos_flag, dup_flag, retain_flag) = Publish::get_flags(bytes)?;
        let (readed_index, remaining_length) = save_remaining_length(&bytes[1..bytes.len()])?;

        let init_variable_header = 1 + readed_index;
        let variable_header = &bytes[init_variable_header..bytes.len()];
        let (topic, packet_id, length) = get_variable_header(variable_header)?;

        let packet_identifier = [packet_id[0], packet_id[1]];
        let payload_to_be_parsed = std::str::from_utf8(&bytes[init_variable_header + length..bytes.len()]);
        let payload = if let Ok(parsed_payload) = payload_to_be_parsed {
            parsed_payload.to_string()
        } else {
            return Err("Unexpected error parsing payload".to_string());
        };

        Ok(Publish {
            _dup: dup_flag,
            retain: retain_flag,
            qos: qos_flag,
            remaining_length,
            topic,
            packet_identifier,
            payload,
            all_bytes: bytes.to_vec(),
        })
    }

    pub fn get_topic(&self) -> String {
        self.topic.to_string()
    }

    pub fn send_response(&self, stream: Sender<StreamType>) -> Result<(), String> {
        match self.qos {
            0x00 => {
                Ok(())
            }
            0x01 => {
                let puback_response = [
                    0x40,
                    0x02,
                    self.packet_identifier[0],
                    self.packet_identifier[1],
                ];

                let sender_stream_result = stream.send((WriteStream, Some(puback_response.to_vec()), None, None));

                if let Err(msg_error) = sender_stream_result {
                    Err(format!("Error in sending response: {}", msg_error))
                } else {
                    Ok(())
                }
            }
            _ => {
                return Err(format!(""))
            }
        }
    }

    pub fn send_message(
        &self,
        sender_topic_manager: Sender<TypeMessage>,
        client_id: String,
    ) -> Result<Self, String> {
        let topic = self.topic.to_owned();

        let publisher_prueba = Publisher::init(
            client_id,
            topic,
            self.all_bytes.clone(),
            self.qos,
            self.retain == 1,
            self.payload.to_string(),
        );

        if let Err(sender_err) = sender_topic_manager.send(TypeMessage::Publisher(publisher_prueba)) {
            return Err(format!("Error sending to publisher_subscriber: {}", sender_err));
        }

        Ok(Publish {
            _dup: self._dup,
            qos: self.qos,
            retain: self.retain,
            remaining_length: self.remaining_length,
            topic: self.topic.clone(),
            packet_identifier: self.packet_identifier,
            payload: self.payload.clone(),
            all_bytes: self.all_bytes.clone(),
        })
    }
}
