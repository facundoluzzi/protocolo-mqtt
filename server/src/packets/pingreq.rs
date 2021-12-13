use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::Sender;

pub fn send_response(sender_stream: Sender<StreamType>) -> Result<(), String> {
    let bytes_response = [0xD0, 0x00];
    let tuple_to_send = (WriteStream, Some(bytes_response.to_vec()), None, None);
    let sender_result = sender_stream.send(tuple_to_send);
    match sender_result {
        Err(err) => Err(err.to_string()),
        Ok(()) => Ok(()),
    }
}
