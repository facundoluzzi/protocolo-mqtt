use crate::packets::packet_manager::PacketManager;
use crate::enums::topic_manager::topic_message::TypeMessage;
use crate::enums::topic_manager::unsubscriber::Unsubscriber;
use crate::helper::remaining_length::save_remaining_length;
use crate::helper::utf8_parser::UTF8;
use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use crate::variable_header::subscribe_variable_header::get_variable_header;

use std::convert::TryInto;
use std::sync::mpsc::Sender;

pub struct Unsubscribe {
    remaining_length: usize,
    packet_identifier: Vec<u8>,
    payload: Vec<u8>,
}

impl Unsubscribe {

    pub fn process_message(bytes: &[u8], packet_manager: &mut PacketManager) -> Result<(), String> {
        let mut unsubscribe = Unsubscribe::init(bytes)?;
        let sender_topic_manager = packet_manager.get_sender_topic_manager();
        let client_id = packet_manager.get_client_id();
        let sender_stream = packet_manager.get_sender_stream();
        unsubscribe = unsubscribe.unsubscribe_topic(sender_topic_manager, client_id)?;
        unsubscribe.send_response(sender_stream)?;
        Ok(())
    }

    pub fn init(bytes: &[u8]) -> Result<Unsubscribe, String> {
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len).unwrap();
        let init_variable_header = 1 + readed_index;
        let variable_header = get_variable_header(&bytes[init_variable_header..bytes.len()])?;
        let (packet_identifier_rec, length) = variable_header;

        let payload = &bytes[init_variable_header + length..bytes.len()];
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

    pub fn unsubscribe_topic(
        &mut self,
        sender: Sender<TypeMessage>,
        client_id: String,
    ) -> Result<Self, String> {
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
            acumulator += length;

            let unsubscriber = Unsubscriber::init(client_id.to_string(), topic.to_string());

            if let Err(sender_err) = sender.send(TypeMessage::Unsubscriber(unsubscriber)) {
                println!("Error sending to publisher_subscriber: {}", sender_err);
            }
        }

        let unsubscribe = Unsubscribe {
            remaining_length: self.remaining_length,
            packet_identifier: self.packet_identifier.clone(),
            payload: self.payload.clone(),
        };

        Ok(unsubscribe)
    }

    pub fn send_response(&self, sender_stream: Sender<StreamType>) -> Result<(), String> {
        let packet_type = 0xB0u8;
        let packet_identifier = self.packet_identifier.clone();
        let bytes_response = vec![
            packet_type,
            0x02u8,
            packet_identifier[0],
            packet_identifier[1],
        ];

        let sender_result = sender_stream.send((WriteStream, Some(bytes_response.to_vec()), None, None));
        if let Err(msg_error) = sender_result {
            Err(format!("Error in sending response: {}", msg_error))
        } else {
            Ok(())
        }
    }
}
