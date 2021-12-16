use std::sync::mpsc::Sender;

use crate::helper::stream::stream_handler::StreamAction::WriteStream;
use crate::helper::stream::stream_handler::StreamType;

pub struct Pingreq {}

impl Pingreq {
    pub fn init() -> Pingreq {
        Pingreq {}
    }

    pub fn send_pingreq(&self, sender_stream: Sender<StreamType>) -> Result<(), String> {
        let result = sender_stream.send((WriteStream, Some(vec![0xC0, 0x00]), None));

        match result {
            Ok(_result_ok) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
