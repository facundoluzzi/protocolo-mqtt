use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::Sender;

pub struct ReconnectStream {
    sender: Sender<StreamType>,
}

impl ReconnectStream {
    pub fn init(sender: Sender<StreamType>) -> Self {
        ReconnectStream { sender }
    }

    pub fn get_sender(&self) -> Sender<StreamType> {
        self.sender.clone()
    }
}
