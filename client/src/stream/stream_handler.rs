use crate::helper::remaining_length::save_remaining_length;
use std::net::Shutdown;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

use std::io::Read;
use std::io::Write;

pub struct Stream {}

pub enum StreamAction {
    WriteStream,
    ReadStream,
    CloseConnectionStream,
}

pub type StreamType = (StreamAction, Option<Vec<u8>>, Option<Sender<Vec<u8>>>);

impl Stream {
    pub fn init(stream: TcpStream) -> Sender<StreamType> {
        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) =
            mpsc::channel();

        let stream_to_write = stream.try_clone().unwrap();
        let stream_to_read = stream.try_clone().unwrap();

        thread::spawn(move || {
            for message_received in receiver_stream {
                let action = message_received.0;
                match action {
                    StreamAction::WriteStream => {
                        if let Some(message) = message_received.1 {
                            Stream::write(stream_to_write.try_clone().unwrap(), message);
                        } else {
                            panic!("Unexpected error: send a Some(message) here");
                        }
                    }
                    StreamAction::ReadStream => {
                        if let Some(sender) = message_received.2 {
                            let stread = stream_to_read.try_clone().unwrap();
                            let stwrite = stream_to_write.try_clone().unwrap();
                            thread::spawn(move || {
                                Stream::read(stread, stwrite, sender);
                            });
                        } else {
                            panic!("Unexpected error: send a Some(Sender<String>) here");
                        }
                    }
                    StreamAction::CloseConnectionStream => {
                        if let Err(_err_msg) = stream_to_read.shutdown(Shutdown::Both) {}

                        if let Err(_err_msg) = stream_to_write.shutdown(Shutdown::Both) {}
                    }
                }
            }
        });

        sender_stream
    }

    fn write(mut stream: TcpStream, message: Vec<u8>) {
        let c: &[u8] = &message; // c: &[u8]
        if let Err(_msg_error) = stream.write(c) {}
    }

    fn read(mut stream: TcpStream, stream_to_write: TcpStream, sender: Sender<Vec<u8>>) {
        let mut data = [0_u8; 5];
        let mut total_data: Vec<u8> = Vec::new();

        let mut is_first_byte = true;
        let mut packet_length = 0;
        let mut readed_bytes = 0;

        while match stream.read(&mut data) {
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
                    println!("BYTES QUE LLEGARON {:?}", bytes_to_process);
                    sender.send(bytes_to_process.to_vec()).unwrap();

                    total_data = Vec::new();

                    false
                } else {
                    true
                }
            }
            Err(_err) => {
                if let Err(_err_msg) = stream.shutdown(Shutdown::Both) {}
                if let Err(_err_msg) = stream_to_write.shutdown(Shutdown::Both) {}
                true
            }
        } {}
    }
}
