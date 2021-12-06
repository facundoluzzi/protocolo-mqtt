use crate::helper::remaining_length::save_remaining_length;
use crate::helper::utf8_parser::UTF8;
use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use crate::topics::topic_types::TypeTopicManager;
use crate::topics::unsubscriber::Unsubscriber;
use crate::variable_header::subscribe_variable_header::get_variable_header;

use std::convert::TryInto;
use std::sync::mpsc::Sender;

pub struct Unsubscribe {
    remaining_length: usize,
    packet_identifier: Vec<u8>,
    payload: Vec<u8>,
}

impl Unsubscribe {
    pub fn init(bytes: &[u8]) -> Result<Unsubscribe, String> {
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len).unwrap();
        let init_variable_header = 1 + readed_index;
        let variable_header = get_variable_header(&bytes[init_variable_header..bytes.len()]);
        let (packet_identifier_rec, length) = variable_header;

        let payload = &bytes[init_variable_header + length..bytes.len()];
        let packet_identifier = packet_identifier_rec[0..2]
            .try_into()
            .expect("slice with incorrect length");
        let unsubscribe = Unsubscribe {
            remaining_length,
            packet_identifier: packet_identifier,
            payload: (*payload).to_vec(),
        };
        Ok(unsubscribe)
    }

    pub fn unsubscribe_topic(
        &mut self,
        sender: Sender<TypeTopicManager>,
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

            if let Err(sender_err) = sender.send(TypeTopicManager::Unsubscriber(unsubscriber)) {
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

    pub fn send_response(&self, sender_stream: Sender<StreamType>) {
        let packet_type = 0xB0u8;
        let packet_identifier = self.packet_identifier.clone();
        let bytes_response = vec![
            packet_type,
            0x02u8,
            packet_identifier[0],
            packet_identifier[1],
        ];

        if let Err(msg_error) =
            sender_stream.send((WriteStream, Some(bytes_response.to_vec()), None, None))
        {
            println!("Error in sending response: {}", msg_error);
        }
    }
}
