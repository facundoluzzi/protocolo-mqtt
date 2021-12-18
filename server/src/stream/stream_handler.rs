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

type ChannelStreamType = (Sender<StreamType>, Receiver<StreamType>);

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
    fn get_streams(stream: TcpStream) -> Result<(TcpStream, TcpStream), String> {
        match (stream.try_clone(), stream.try_clone()) {
            (Ok(stream_to_read), Ok(stream_to_write)) => Ok((stream_to_read, stream_to_write)),
            _ => Err("Unexpected error".to_string()),
        }
    }

    fn get_cloned_stream(stream: Result<TcpStream, std::io::Error>) -> Result<TcpStream, String> {
        match stream {
            Ok(stream) => Ok(stream),
            Err(_err) => Err("Unexpected error cloning stream".to_string()),
        }
    }

    fn process_input_and_output(
        to_write: TcpStream,
        to_read: TcpStream,
        receiver: Receiver<StreamType>,
        logger: Logger,
    ) -> Result<(), String> {
        for message_received in receiver {
            let action = message_received.0;
            let cloned_to_write = Stream::get_cloned_stream(to_write.try_clone())?;
            let cloned_to_read = Stream::get_cloned_stream(to_read.try_clone())?;
            match action {
                StreamAction::WriteStream => {
                    Stream::write(cloned_to_write, message_received.1, logger.clone());
                }
                StreamAction::ReadStream => {
                    if let Some(sender) = message_received.2 {
                        let cloned_logger = logger.clone();
                        thread::spawn(move || {
                            Stream::read(cloned_to_read, cloned_to_write, sender, cloned_logger);
                        });
                    } else {
                        panic!("Unexpected error: send a Some(Sender<String>) here");
                    }
                }
                StreamAction::CloseConnectionStream => {
                    Stream::close_streams(cloned_to_write, cloned_to_read, logger.clone())
                }
                StreamAction::SetKeepAliveStream => {
                    if let Err(err) = cloned_to_read.set_read_timeout(message_received.3) {
                        println!("Unexpected error setting keep alive: {}", err);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn init(stream_received: TcpStream, logger: Logger) -> Result<Sender<StreamType>, String> {
        let (sender_stream, receiver_stream): ChannelStreamType = mpsc::channel();
        let (stream_to_read, stream_to_write) = Stream::get_streams(stream_received)?;
        std::thread::spawn(move || {
            let result_process = Stream::process_input_and_output(
                stream_to_write,
                stream_to_read,
                receiver_stream,
                logger,
            );
            if let Err(err) = result_process {
                println!("{}", err);
            }
        });
        Ok(sender_stream)
    }

    fn write(mut stream: TcpStream, message: Option<Vec<u8>>, mut logger: Logger) {
        match message {
            Some(message) => {
                let message_to_write: &[u8] = &message;
                if let Err(msg_error) = stream.write(message_to_write) {
                    logger.info(format!("Error in sending response: {}", msg_error));
                }
            }
            None => panic!("Unexpected error: send a Some(message) here"),
        };
    }

    fn read(
        mut stream: TcpStream,
        stream_to_write: TcpStream,
        sender: Sender<Vec<u8>>,
        logger: Logger,
    ) {
        let mut total_data: Vec<u8> = Vec::new();
        let mut is_first_byte = true;
        let mut packet_length = 0;
        let mut readed_bytes = 0;
        let mut data = [0_u8; 5];
        while match stream.read(&mut data) {
            Ok(size) => {
                total_data = Stream::build_total_data(
                    &mut data,
                    total_data.clone(),
                    &mut is_first_byte,
                    size,
                    &mut packet_length,
                    &mut readed_bytes,
                );
                Stream::finish_to_process_packet(
                    &mut total_data,
                    &mut packet_length,
                    &mut readed_bytes,
                    sender.clone(),
                )
            }
            Err(_err) => {
                Stream::close_streams(
                    stream.try_clone().unwrap(),
                    stream_to_write.try_clone().unwrap(),
                    logger.clone(),
                );
                true
            }
        } {}
    }

    fn build_total_data(
        data: &mut [u8; 5],
        total_data: Vec<u8>,
        is_first_byte: &mut bool,
        size: usize,
        packet_length: &mut usize,
        readed_bytes: &mut usize,
    ) -> Vec<u8> {
        if *is_first_byte && size != 0 {
            let (_readed_bytes, _packet_length) = save_remaining_length(&data[1..size]).unwrap();
            *packet_length = _packet_length;
            *readed_bytes = _readed_bytes;
            *is_first_byte = false;
            [total_data, data.to_vec()].concat()
        } else if !*is_first_byte && size != 0 {
            [total_data, data.to_vec()].concat()
        } else {
            total_data
        }
    }

    fn finish_to_process_packet(
        total_data: &mut Vec<u8>,
        packet_length: &mut usize,
        readed_bytes: &mut usize,
        sender: Sender<Vec<u8>>,
    ) -> bool {
        if total_data.len() > *packet_length + *readed_bytes {
            let bytes_to_process = &total_data[0..*packet_length + *readed_bytes + 1];
            sender.send(bytes_to_process.to_vec()).unwrap();
            *total_data = Vec::new();
            false
        } else {
            true
        }
    }

    fn close_streams(stream_to_write: TcpStream, stream_to_read: TcpStream, mut logger: Logger) {
        if let Err(err_msg) = stream_to_read.shutdown(Shutdown::Both) {
            logger.info(format!("Unexpected error closing the stream: {}", err_msg));
        }

        if let Err(err_msg) = stream_to_write.shutdown(Shutdown::Both) {
            logger.info(format!("Unexpected error closing the stream: {}", err_msg));
        }
    }
}
