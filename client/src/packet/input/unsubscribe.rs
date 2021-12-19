use crate::helper::packet_builder::build_bytes_for_unsubscribe;
use crate::helper::stream::stream_handler::StreamAction::WriteStream;
use crate::types::StreamType;
use std::sync::mpsc::Sender;

pub struct Unsubscribe {
    topic: String,
}

impl Unsubscribe {
    pub fn init(topic: String) -> Unsubscribe {
        Unsubscribe { topic }
    }

    /// Obtiene la serie de bytes que van a representar al paquete Unsuscribe, y manda una accion de escritura hacia el Stream, para que
    /// lo mande hacia el broker
    pub fn send_unsubscribe(&self, sender_stream: Sender<StreamType>) -> Result<(), String> {
        let unsubscribe_bytes = build_bytes_for_unsubscribe(self.topic.to_string());
        let result = sender_stream.send((WriteStream, Some(unsubscribe_bytes), None));

        match result {
            Ok(_result_ok) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
