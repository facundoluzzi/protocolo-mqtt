use crate::helper::remaining_length::save_remaining_length;
use std::net::Shutdown;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

use std::io::Read;
use std::io::Write;

pub struct Stream {
    stream: TcpStream,
}

pub enum StreamAction {
    WriteStream,
    ReadStream,
    CloseConnectionStream,
}

pub type StreamType = (StreamAction, Option<Vec<u8>>, Option<Sender<Vec<u8>>>);

impl Stream {
    pub fn init(stream_received: TcpStream) -> Sender<StreamType> {
        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) =
            mpsc::channel();

        let mut stream = Stream {
            stream: stream_received,
        };

        thread::spawn(move || {
            for message_received in receiver_stream {
                let action = message_received.0;
                match action {
                    StreamAction::WriteStream => {
                        if let Some(message) = message_received.1 {
                            stream.write(message);
                        } else {
                            panic!("Unexpected error: send a Some(message) here");
                        }
                    }
                    StreamAction::ReadStream => {
                        if let Some(sender) = message_received.2 {
                            stream.read(sender);
                        } else {
                            panic!("Unexpected error: send a Some(Sender<String>) here");
                        }
                    }
                    StreamAction::CloseConnectionStream => {
                        if let Err(err_msg) = stream.stream.shutdown(Shutdown::Both) {
                            println!("Unexpected error closing the stream: {}", err_msg);
                        }
                    }
                }
            }
        });

        sender_stream
    }

    fn write(&mut self, message: Vec<u8>) {
        let c: &[u8] = &message; // c: &[u8]
        if let Err(msg_error) = self.stream.write(c) {
            println!("Error in sending response: {}", msg_error);
        }
    }

    fn read(&mut self, sender: Sender<Vec<u8>>) {
        let mut data = [0_u8; 5];
        let mut total_data: Vec<u8> = Vec::new();

        let mut is_first_byte = true;
        let mut packet_length = 0;
        let mut readed_bytes = 0;

        while match self.stream.read(&mut data) {
            Ok(size) => {
                if is_first_byte && size != 0 {
                    let (_readed_bytes, _packet_length) =
                        save_remaining_length(&data[1..size]).unwrap();
                    packet_length = _packet_length;
                    readed_bytes = _readed_bytes;

                    is_first_byte = false;
                    total_data = [total_data, data.to_vec()].concat();
                } else if !is_first_byte && size != 0 {
                    total_data = [total_data, data.to_vec()].concat();
                }

                if total_data.len() > packet_length + readed_bytes {
                    is_first_byte = true;
                    let bytes_to_process = &total_data[0..packet_length + readed_bytes + 1];
                    sender.send(bytes_to_process.to_vec()).unwrap();

                    total_data = Vec::new();
                    false
                } else {
                    true
                }
            }
            Err(err) => {
                let err_msg = err.to_string().as_bytes().to_vec();
                sender.send(err_msg).unwrap();
                println!("Falle: {}", err);
                true
                // logger.error(format!(
                //     "An error occurred, terminating connection with {}",
                //     stream.peer_addr().unwrap()
                // ));
                // stream.shutdown(Shutdown::Both).unwrap();
                // true
            }
        } {}
    }
}
