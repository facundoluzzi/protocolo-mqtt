use crate::packet_builder::build_bytes_for_publish;
use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::Sender;

pub struct Publish {
    message: String,
    topic: String,
    is_qos_0: bool,
}

impl Publish {
    pub fn init(message: String, topic: String, is_qos_0: bool) -> Publish {
        Publish {
            message,
            topic,
            is_qos_0,
        }
    }

    pub fn send_publish(&self, sender_stream: Sender<StreamType>) -> Result<(), String> {
        let publish_bytes = build_bytes_for_publish(
            self.topic.to_string(),
            self.message.to_string(),
            self.is_qos_0,
        );

        let result = sender_stream.send((WriteStream, Some(publish_bytes), None));

        match result {
            Ok(_result_ok) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
