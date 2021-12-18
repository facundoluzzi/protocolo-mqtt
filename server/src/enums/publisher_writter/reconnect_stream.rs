use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::Sender;

/// Contiene el sender utilizado para que el publisher writer envíe el mensaje al stream cuando se reconecta
pub struct ReconnectStream {
    sender: Sender<StreamType>,
}

impl ReconnectStream {

    /// Constructor del struct
    pub fn init(sender: Sender<StreamType>) -> Self {
        ReconnectStream { sender }
    }

    /// Devuelve el sender
    pub fn get_sender(&self) -> Sender<StreamType> {
        self.sender.clone()
    }
}
