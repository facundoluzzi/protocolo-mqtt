use crate::packet_builder::build_bytes_for_suscribe;
use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::Sender;

pub struct Subscribe {
    topic: String,
    is_qos_0: bool,
}

impl Subscribe {
    pub fn init(topic: String, is_qos_0: bool) -> Subscribe {
        Subscribe { topic, is_qos_0 }
    }

    pub fn send_suscribe(&self, sender_stream: Sender<StreamType>) -> Result<(), String> {
        let suscribe_bytes = build_bytes_for_suscribe(self.topic.to_string(), self.is_qos_0);
        let result = sender_stream.send((WriteStream, Some(suscribe_bytes), None));

        match result {
            Ok(_result_ok) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
