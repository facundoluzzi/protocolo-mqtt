use crate::enums::topic_manager::topic_message::TypeMessage;
use crate::enums::topic_manager::unsubscriber::Unsubscriber;
use crate::enums::wildcard::wildcard_result::WildcardResult::{
    HasNoWildcard, HasWildcard, InvalidWildcard,
};
use crate::helper::remaining_length::save_remaining_length;
use crate::helper::utf8_parser::UTF8;
use crate::helper::validate_payload::check_payload;
use crate::helper::validate_reserved_bytes::check_reserved_bytes;
use crate::packets::packet_manager::PacketManager;
use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use crate::variable_header::subscribe_variable_header::get_variable_header;
use crate::wildcard::verify_wildcard;
use crate::wildcard::wildcard_handler::Wildcard;
use std::convert::TryInto;
use std::sync::mpsc::Sender;

pub struct Unsubscribe {
    remaining_length: usize,
    packet_identifier: Vec<u8>,
    payload: Vec<u8>,
}

impl Unsubscribe {
    /// Recibe los bytes del paquete y el packet manager.
    /// Desubscribe los topicos recibidos.
    /// Devuelve Ok(()) o un Err de typo String en caso de que algo falle.
    pub fn process_message(bytes: &[u8], packet_manager: &mut PacketManager) -> Result<(), String> {
        let mut unsubscribe = Unsubscribe::init(bytes)?;
        let sender_stream = packet_manager.get_sender_stream();
        unsubscribe = unsubscribe.unsubscribe_topic(packet_manager)?;
        unsubscribe.send_response(sender_stream)?;
        Ok(())
    }

    /// Constructor del struct
    pub fn init(bytes: &[u8]) -> Result<Unsubscribe, String> {
        check_reserved_bytes(bytes[0])?;
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len).unwrap();
        let init_variable_header = 1 + readed_index;
        let variable_header = get_variable_header(&bytes[init_variable_header..bytes.len()])?;
        let (packet_identifier_rec, length) = variable_header;

        let payload = check_payload(&bytes[init_variable_header + length..bytes.len()])?;
        let packet_identifier = packet_identifier_rec[0..2]
            .try_into()
            .expect("slice with incorrect length");
        let unsubscribe = Unsubscribe {
            remaining_length,
            packet_identifier,
            payload: (*payload).to_vec(),
        };
        Ok(unsubscribe)
    }

    fn send_unsubscribe_to_topic_manager(
        &mut self,
        topics: Vec<String>,
        packet_manager: &PacketManager,
    ) {
        topics.into_iter().for_each(|topic| {
            match verify_wildcard::get_wilcard(topic.to_owned()) {
                HasWildcard(wildcard) => {
                    self.process_topic_with_wildcard(topic, Some(wildcard), packet_manager)
                }
                HasNoWildcard => self.process_topic_without_wildcard(topic, packet_manager),
                InvalidWildcard => {}
            };
        });
    }

    fn send_to_topic_manager(&self, packet_manager: &PacketManager, unsubscriber: Unsubscriber) {
        let sender_topic_manager = packet_manager.get_sender_topic_manager();
        if let Err(err) = sender_topic_manager.send(TypeMessage::Unsubscriber(unsubscriber)) {
            println!("{:?}", err);
        }
    }

    fn process_topic_with_wildcard(
        &self,
        topic: String,
        wildcard: Option<Wildcard>,
        packet_manager: &PacketManager,
    ) {
        let client_id = packet_manager.get_client_id();
        let unsubscriber = Unsubscriber::init(client_id, topic, wildcard);
        self.send_to_topic_manager(packet_manager, unsubscriber);
    }

    fn process_topic_without_wildcard(&self, topic: String, packet_manager: &PacketManager) {
        let client_id = packet_manager.get_client_id();
        let unsubscriber = Unsubscriber::init(client_id, topic, None);
        self.send_to_topic_manager(packet_manager, unsubscriber);
    }

    /// elimina la suscripción del usuario en el tópico recibido
    pub fn unsubscribe_topic(&mut self, packet_manager: &PacketManager) -> Result<Self, String> {
        let mut acumulator: usize = 0;
        let mut topics: Vec<String> = Vec::new();

        while self.payload.len() > acumulator {
            let topic_bytes = &self.payload[acumulator..self.payload.len()];
            let (topic, length) = UTF8::utf8_parser(topic_bytes)?;
            topics.push(topic);
            acumulator += length;
        }
        self.send_unsubscribe_to_topic_manager(topics, packet_manager);

        let unsubscribe = Unsubscribe {
            remaining_length: self.remaining_length,
            packet_identifier: self.packet_identifier.clone(),
            payload: self.payload.clone(),
        };

        Ok(unsubscribe)
    }

    /// envia el unsubscribe al cliente
    pub fn send_response(&self, sender_stream: Sender<StreamType>) -> Result<(), String> {
        let packet_type = 0xB0u8;
        let packet_identifier = self.packet_identifier.clone();
        let bytes_response = vec![
            packet_type,
            0x02u8,
            packet_identifier[0],
            packet_identifier[1],
        ];

        let sender_result =
            sender_stream.send((WriteStream, Some(bytes_response.to_vec()), None, None));
        if let Err(msg_error) = sender_result {
            Err(format!("Error in sending response: {}", msg_error))
        } else {
            Ok(())
        }
    }
}
