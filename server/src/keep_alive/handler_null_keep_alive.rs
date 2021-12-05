use crate::stream::stream_handler::StreamAction::SetKeepAliveStream;
use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::Sender;

pub fn init(sender: Sender<StreamType>) -> Result<(), String> {
    match sender.send((SetKeepAliveStream, None, None, None)) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
