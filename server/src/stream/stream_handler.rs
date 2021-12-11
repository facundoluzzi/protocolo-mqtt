use crate::helper::remaining_length::save_remaining_length;
use crate::logs::logger::Logger;
use std::net::Shutdown;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use std::io::Read;
use std::io::Write;

pub struct Stream {}

pub enum StreamAction {
    WriteStream,
    ReadStream,
    CloseConnectionStream,
    SetKeepAliveStream,
}

pub type StreamType = (
    StreamAction,
    Option<Vec<u8>>,
    Option<Sender<Vec<u8>>>,
    Option<Duration>,
);

impl Stream {
    pub fn init(stream_received: TcpStream, mut logger: Logger) -> Sender<StreamType> {
        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) =
            mpsc::channel();

        let stream_to_read = stream_received.try_clone().unwrap();
        let stream_to_write = stream_received.try_clone().unwrap();

        thread::spawn(move || {
            for message_received in receiver_stream {
                let action = message_received.0;
                match action {
                    StreamAction::WriteStream => {
                        if let Some(message) = message_received.1 {
                            let cloned_logger = logger.clone();
                            Stream::write(
                                stream_to_write.try_clone().unwrap(),
                                message,
                                cloned_logger,
                            );
                        } else {
                            panic!("Unexpected error: send a Some(message) here");
                        }
                    }
                    StreamAction::ReadStream => {
                        logger.info("empezó a leer".to_string());
                        if let Some(sender) = message_received.2 {
                            let stread = stream_to_read.try_clone().unwrap();
                            let stwrite = stream_to_write.try_clone().unwrap();
                            let cloned_logger = logger.clone();
                            thread::spawn(move || {
                                Stream::read(stread, stwrite, sender, cloned_logger);
                            });
                            logger.info("termino de leer".to_string());
                        } else {
                            panic!("Unexpected error: send a Some(Sender<String>) here");
                        }
                    }
                    StreamAction::CloseConnectionStream => {
                        if let Err(err_msg) = stream_to_read.shutdown(Shutdown::Both) {
                            logger
                                .info(format!("Unexpected error closing the stream: {}", err_msg));
                        }

                        if let Err(err_msg) = stream_to_write.shutdown(Shutdown::Both) {
                            logger
                                .info(format!("Unexpected error closing the stream: {}", err_msg));
                        }
                    }
                    StreamAction::SetKeepAliveStream => {
                        stream_to_read.set_read_timeout(message_received.3).unwrap();
                    }
                }
            }
        });

        sender_stream
    }

    fn write(mut stream: TcpStream, message: Vec<u8>, mut logger: Logger) {
        let c: &[u8] = &message; // c: &[u8]
        if let Err(msg_error) = stream.write(c) {
            logger.info(format!("Error in sending response: {}", msg_error));
        } else {
            println!("Terminando de escribir los bytes: {:?}", c);
        }
    }

    fn read(
        mut stream: TcpStream,
        stream_to_write: TcpStream,
        sender: Sender<Vec<u8>>,
        mut logger: Logger,
    ) {
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
                    println!("Bytes terminados de leer: {:?}", bytes_to_process);
                    sender.send(bytes_to_process.to_vec()).unwrap();

                    total_data = Vec::new();

                    false
                } else {
                    true
                }
            }
            Err(_err) => {
                if let Err(err_msg) = stream.shutdown(Shutdown::Both) {
                    logger.info(format!("Unexpected error closing the stream: {}", err_msg));
                }

                if let Err(err_msg) = stream_to_write.shutdown(Shutdown::Both) {
                    logger.info(format!("Unexpected error closing the stream: {}", err_msg));
                }

                // logger.error(format!(
                //     "An error occurred, terminating connection with {}",
                //     stream.peer_addr().unwrap()
                // ));

                true
            }
        } {}
    }
}
