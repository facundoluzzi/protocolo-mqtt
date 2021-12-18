use std::sync::mpsc::Sender;

use crate::helper::stream::stream_handler::StreamAction::WriteStream;
use crate::types::StreamType;

pub struct PubackToSend {
    packet_identifier: Vec<u8>,
}

impl PubackToSend {
    pub fn init(packet_identifier: Vec<u8>) -> PubackToSend {
        PubackToSend { packet_identifier }
    }

    pub fn send_puback(&self, sender_stream: Sender<StreamType>) -> Result<(), String> {
        let puback_bytes = vec![
            0x40,
            0x02,
            self.packet_identifier[0],
            self.packet_identifier[1],
        ];

        let result = sender_stream.send((WriteStream, Some(puback_bytes), None));

        match result {
            Ok(_result_ok) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
