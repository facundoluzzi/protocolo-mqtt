use std::sync::mpsc::Sender;

use crate::helper::stream::stream_handler::StreamAction::WriteStream;
use crate::types::StreamType;

pub struct Pingreq {
    interval: i32,
}

impl Pingreq {
    pub fn init(interval: i32) -> Pingreq {
        Pingreq { interval }
    }

    /// Obtiene la serie de bytes que van a representar al paquete Pingreq, y manda una accion de escritura hacia el Stream, para que
    /// lo mande hacia el broker
    pub fn send_pingreq(&self, sender_stream: Sender<StreamType>) -> Result<(), String> {
        let result = sender_stream.send((WriteStream, Some(vec![0xC0, 0x00]), None));

        match result {
            Ok(_result_ok) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    /// Devuelve el intervalo de tiempo que representa la frecuencia del mandado del paquete pingreq
    pub fn get_interval(&self) -> i32 {
        self.interval
    }
}
