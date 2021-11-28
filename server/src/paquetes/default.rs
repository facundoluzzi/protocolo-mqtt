use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::Sender;

pub struct Default {}

impl Default {
    pub fn init(_bytes: &[u8]) -> Default {
        Default {}
    }

    pub fn get_type(&self) -> String {
        "default".to_owned()
    }

    pub fn send_response(&self, stream: Sender<StreamType>) {
        if let Err(msg_error) =
            stream.send((WriteStream, Some(b"default message\n".to_vec()), None))
        {
            println!("Error in sending response: {}", msg_error);
        }
    }

    pub fn send_message(&self, _stream: &Sender<String>) {
        //todo
    }
}
