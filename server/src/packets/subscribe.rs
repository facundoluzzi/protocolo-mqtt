use crate::enums::topic_manager::subscriber::Subscriber;
use crate::enums::topic_manager::topic_message::TypeMessage;
use crate::enums::wildcard::wildcard_result::WildcardResult::{
    HasNoWildcard, HasWildcard, InvalidWildcard,
};
use crate::helper::remaining_length::save_remaining_length;
use crate::helper::utf8_parser::UTF8;
use crate::packets::packet_manager::PacketManager;
use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::variable_header::subscribe_variable_header::get_variable_header;
use crate::wildcard::verify_wildcard;
use crate::wildcard::wildcard_handler::Wildcard;

use std::convert::TryInto;

pub struct Subscribe {
    remaining_length: usize,
    packet_identifier: [u8; 2],
    payload: Vec<u8>,
    return_codes: Vec<u8>,
}

impl Subscribe {
    pub fn process_message(bytes: &[u8], packet_manager: &PacketManager) -> Result<(), String> {
        let mut subscribe = Subscribe::init(bytes)?;
        let subscribe_topic_response = subscribe.subscribe_topic(packet_manager)?;
        subscribe_topic_response.send_response(packet_manager)?;
        Ok(())
    }

    pub fn init(bytes: &[u8]) -> Result<Subscribe, String> {
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len)?;

        let init_variable_header = 1 + readed_index;

        let variable_header = get_variable_header(&bytes[init_variable_header..bytes.len()])?;
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

    fn send_to_topic_manager(
        &mut self,
        packet_manager: &PacketManager,
        qos: u8,
        subscriber: Subscriber,
    ) {
        let sender_topic_manager = packet_manager.get_sender_topic_manager();

        match sender_topic_manager.send(TypeMessage::Subscriber(subscriber)) {
            Ok(_) => self.return_codes.push(qos),
            Err(_) => self.return_codes.push(0x80),
        }
    }

    fn process_topic_with_wildcard(
        &mut self,
        topic: String,
        qos: u8,
        wildcard: Option<Wildcard>,
        packet_manager: &PacketManager,
    ) {
        let client_id = packet_manager.get_client_id();
        let sender_user_manager = packet_manager.get_sender_user_manager();
        let subscriber = Subscriber::init(client_id, topic, sender_user_manager, wildcard, qos);
        self.send_to_topic_manager(packet_manager, qos, subscriber);
    }

    fn process_topic_without_wildcard(
        &mut self,
        topic: String,
        qos: u8,
        packet_manager: &PacketManager,
    ) {
        let client_id = packet_manager.get_client_id();
        let sender_user_manager = packet_manager.get_sender_user_manager();
        let subscriber = Subscriber::init(client_id, topic, sender_user_manager, None, qos);
        self.send_to_topic_manager(packet_manager, qos, subscriber);
    }

    fn send_subscribe_to_topic_manager(
        &mut self,
        topics: Vec<(String, u8)>,
        packet_manager: &PacketManager,
    ) {
        topics.into_iter().for_each(|(topic, qos)| {
            if qos > 1 {
                self.return_codes.push(0x80);
                return;
            }
            match verify_wildcard::get_wilcard(topic.to_owned()) {
                HasWildcard(wildcard) => {
                    self.process_topic_with_wildcard(topic, qos, Some(wildcard), packet_manager)
                }
                HasNoWildcard => self.process_topic_without_wildcard(topic, qos, packet_manager),
                InvalidWildcard => self.return_codes.push(0x80),
            };
            return;
        });
    }

    pub fn subscribe_topic(&mut self, packet_manager: &PacketManager) -> Result<Self, String> {
        let mut acumulator: usize = 0;
        let mut topics_qos: Vec<(String, u8)> = Vec::new();

        while self.payload.len() > acumulator {
            let topic_qos = &self.payload[acumulator..self.payload.len()];
            let (topic, length) = UTF8::utf8_parser(topic_qos)?;
            let qos = self.payload[length + acumulator];
            topics_qos.push((topic, qos));
            acumulator += length + 1;
        }

        self.send_subscribe_to_topic_manager(topics_qos, packet_manager);

        let subscribe = Subscribe {
            remaining_length: self.remaining_length,
            packet_identifier: self.packet_identifier,
            payload: self.payload.clone(),
            return_codes: self.return_codes.clone(),
        };

        Ok(subscribe)
    }

    pub fn send_response(&self, packet_manager: &PacketManager) -> Result<(), String> {
        let sender_stream = packet_manager.get_sender_stream();
        let mut bytes_response = vec![
            0x90,
            0x02,
            self.packet_identifier[0],
            self.packet_identifier[1],
        ];

        for return_code in &self.return_codes {
            bytes_response.push(*return_code);
            bytes_response[1] += 1;
        }

        let sender_response =
            sender_stream.send((WriteStream, Some(bytes_response.to_vec()), None, None));
        if let Err(msg_error) = sender_response {
            return Err(format!("Error in sending response: {}", msg_error));
        } else {
            Ok(())
        }
    }
}
