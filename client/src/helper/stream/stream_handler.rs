use crate::helper::remaining_length::save_remaining_length;
use crate::types::StreamType;
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

impl Stream {
    pub fn init(stream: TcpStream) -> Result<Sender<StreamType>, std::io::Error> {
        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) =
            mpsc::channel();

        thread::spawn(move || -> Result<(), std::io::Error> {
            for message_received in receiver_stream {
                Stream::match_action_for_stream(
                    message_received,
                    stream.try_clone()?,
                    stream.try_clone()?,
                )?;
            }
            Ok(())
        });

        Ok(sender_stream)
    }

    fn match_action_for_stream(
        message_received: StreamType,
        stream_to_write: TcpStream,
        stream_to_read: TcpStream,
    ) -> Result<(), std::io::Error> {
        let action = message_received.0;
        match action {
            StreamAction::WriteStream => {
                if let Some(message) = message_received.1 {
                    Stream::write(stream_to_write, message);
                    Ok(())
                } else {
                    panic!("Unexpected error: send a Some(message) here");
                }
            }
            StreamAction::ReadStream => {
                if let Some(sender) = message_received.2 {
                    thread::spawn(move || {
                        Stream::read(stream_to_read, stream_to_write, sender);
                    });
                    Ok(())
                } else {
                    panic!("Unexpected error: send a Some(Sender<String>) here");
                }
            }
            StreamAction::CloseConnectionStream => {
                if let Err(_err_msg) = stream_to_read.shutdown(Shutdown::Both) {
                    return Err(_err_msg);
                }

                if let Err(_err_msg) = stream_to_write.shutdown(Shutdown::Both) {
                    return Err(_err_msg);
                }
                Ok(())
            }
        }
    }

    fn write(mut stream: TcpStream, message: Vec<u8>) {
        let c: &[u8] = &message; // c: &[u8]
        if let Err(_msg_error) = stream.write(c) {}
    }

    fn process_first_byte_of_packet(
        data: [u8; 5],
        size: usize,
        is_first_byte: &mut bool,
        packet_length: &mut usize,
        readed_bytes: &mut usize,
        mut total_data: Vec<u8>,
    ) -> Vec<u8> {
        if let Ok((_readed_bytes, _packet_length)) = save_remaining_length(&data[1..size]) {
            *packet_length = _packet_length;
            *readed_bytes = _readed_bytes;
        }

        *is_first_byte = false;
        total_data = [total_data, data.to_vec()].concat();
        total_data
    }

    fn process_total_bytes_of_packet(
        is_first_byte: &mut bool,
        total_data: &mut Vec<u8>,
        packet_length: usize,
        readed_bytes: usize,
        sender: Sender<Vec<u8>>,
    ) -> bool {
        *is_first_byte = true;
        let bytes_to_process = &total_data[0..packet_length + readed_bytes + 1];
        if sender.send(bytes_to_process.to_vec()).is_err() {
            return false;
        }

        *total_data = Vec::new();
        false
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
                    total_data = Stream::process_first_byte_of_packet(
                        data,
                        size,
                        &mut is_first_byte,
                        &mut packet_length,
                        &mut readed_bytes,
                        total_data.clone(),
                    );
                } else if !is_first_byte && size != 0 {
                    total_data = [total_data, data.to_vec()].concat();
                }

                if total_data.len() > packet_length + readed_bytes {
                    Stream::process_total_bytes_of_packet(
                        &mut is_first_byte,
                        &mut total_data,
                        packet_length,
                        readed_bytes,
                        sender.clone(),
                    )
                } else if is_first_byte && size == 0 {
                    if sender.send(vec![]).is_err() {
                        return;
                    }
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
