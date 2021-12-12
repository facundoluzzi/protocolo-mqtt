use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::Sender;

pub fn send_response(sender_stream: Sender<StreamType>) {
    let bytes_response = [0xE0, 0x00];
    let tuple_to_send = (WriteStream, Some(bytes_response.to_vec()), None, None);
    if let Err(msg_error) = sender_stream.send(tuple_to_send) {
        println!("Error in sending response: {}", msg_error);
    }
}
