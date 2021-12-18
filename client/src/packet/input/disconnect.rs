use crate::helper::packet_builder::build_bytes_for_disconnect;
use crate::helper::stream::stream_handler::StreamAction::WriteStream;
use crate::types::StreamType;
use std::sync::mpsc::Sender;

pub struct Disconnect {}

impl Disconnect {
    pub fn init() -> Disconnect {
        Disconnect {}
    }

    pub fn send_disconnect(&self, sender_stream: Sender<StreamType>) -> Result<(), String> {
        let disconnect_bytes = build_bytes_for_disconnect();
        if sender_stream
            .send((WriteStream, Some(disconnect_bytes), None))
            .is_err()
        {
            return Err("Problema mandando paquete disconnect".to_string());
        }

        Ok(())
    }
}
