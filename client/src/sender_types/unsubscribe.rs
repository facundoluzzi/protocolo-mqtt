use crate::packet_builder::build_bytes_for_unsubscribe;
use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::Sender;

pub struct Unsubscribe {
    topic: String,
}

impl Unsubscribe {
    pub fn init(topic: String) -> Unsubscribe {
        Unsubscribe { topic }
    }

    pub fn send_unsubscribe(&self, sender_stream: Sender<StreamType>) -> Result<(), String> {
        let unsubscribe_bytes = build_bytes_for_unsubscribe(self.topic.to_string());
        let result = sender_stream.send((WriteStream, Some(unsubscribe_bytes), None));

        match result {
            Ok(_result_ok) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
