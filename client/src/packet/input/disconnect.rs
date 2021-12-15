use crate::helper::packet_builder::build_bytes_for_disconnect;
use crate::helper::stream::stream_handler::StreamAction::WriteStream;
use crate::helper::stream::stream_handler::StreamType;
use std::sync::mpsc::Sender;

pub struct Disconnect {}

impl Disconnect {
    pub fn init() -> Disconnect {
        Disconnect {}
    }

    pub fn send_disconnect(&self, sender_stream: Sender<StreamType>) -> Result<(), String> {
        let disconnect_bytes = build_bytes_for_disconnect();
        sender_stream
            .send((WriteStream, Some(disconnect_bytes), None))
            .unwrap();

        Ok(())
    }
}
