use crate::helper::packet_builder::build_bytes_for_publish;
use crate::helper::stream::stream_handler::StreamAction::WriteStream;
use crate::types::StreamType;
use std::sync::mpsc::Sender;

pub struct Publish {
    message: String,
    topic: String,
    is_qos_0: bool,
    retain_is_active: bool,
}

impl Publish {
    pub fn init(message: String, topic: String, is_qos_0: bool, retain_is_active: bool) -> Publish {
        Publish {
            message,
            topic,
            is_qos_0,
            retain_is_active,
        }
    }

    /// Obtiene la serie de bytes que van a representar al paquete Publish, y manda una accion de escritura hacia el Stream, para que
    /// lo mande hacia el broker
    pub fn send_publish(&self, sender_stream: Sender<StreamType>) -> Result<(), String> {
        let publish_bytes = build_bytes_for_publish(
            self.topic.to_string(),
            self.message.to_string(),
            self.is_qos_0,
            self.retain_is_active,
        );

        let result = sender_stream.send((WriteStream, Some(publish_bytes), None));

        match result {
            Ok(_result_ok) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
