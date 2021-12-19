use crate::helper::packet_builder::build_bytes_for_suscribe;
use crate::helper::stream::stream_handler::StreamAction::WriteStream;
use crate::types::StreamType;
use std::sync::mpsc::Sender;

pub struct Subscribe {
    list_of_topics: Vec<(String, bool)>,
}

impl Subscribe {
    pub fn init(list_of_topics: Vec<(String, bool)>) -> Subscribe {
        Subscribe { list_of_topics }
    }

    /// Obtiene la serie de bytes que van a representar al paquete Subscribe, y manda una accion de escritura hacia el Stream, para que
    /// lo mande hacia el broker
    pub fn send_suscribe(self, sender_stream: Sender<StreamType>) -> Result<(), String> {
        let subscribe_bytes = build_bytes_for_suscribe(self.list_of_topics);
        let result = sender_stream.send((WriteStream, Some(subscribe_bytes), None));

        match result {
            Ok(_result_ok) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
