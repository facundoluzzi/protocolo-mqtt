use crate::helper::remaining_length::save_remaining_length;
use crate::logs::logger::Logger;
use std::io::Error;
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
    fn process_input_and_output(
        message_received: StreamType,
        to_write: TcpStream,
        to_read: TcpStream,
    ) -> Result<(), Error> {
        let action = message_received.0;
        match action {
            StreamAction::WriteStream => Stream::write(to_write, message_received.1),
            StreamAction::ReadStream => {
                if let Some(sender) = message_received.2 {
                    thread::spawn(move || -> Result<(), Error> {
                        Stream::read(to_read, to_write, sender)
                    });
                    Ok(())
                } else {
                    panic!("Unexpected error: send a Some(Sender<String>) here");
                }
            }
            StreamAction::CloseConnectionStream => Stream::close_streams(to_write, to_read),
            StreamAction::SetKeepAliveStream => to_read.set_read_timeout(message_received.3),
        }
    }

    /// Recibe un tcp stream para poder escribir y leer al mismo tiempo. Lanza un thread que se queda escuchando eventos
    /// de diferentes partes de la app
    pub fn init(
        stream_received: TcpStream,
        mut logger: Logger,
    ) -> Result<Sender<StreamType>, Error> {
        let (sender_stream, receiver_stream): ChannelStreamType = mpsc::channel();
        thread::spawn(move || -> Result<(), Error> {
            for message_received in receiver_stream {
                if let Err(err) = Stream::process_input_and_output(
                    message_received,
                    stream_received.try_clone()?,
                    stream_received.try_clone()?,
                ) {
                    logger.info(format!("Unexpected error in logger: {}", err.to_string()));
                    return Err(err);
                }
            }
            Ok(())
        });
        Ok(sender_stream)
    }

    fn write(mut stream: TcpStream, message: Option<Vec<u8>>) -> Result<(), Error> {
        match message {
            Some(message) => {
                let message_to_write: &[u8] = &message;
                stream.write_all(message_to_write)?;
                Ok(())
            }
            None => panic!("Unexpected error: send a Some(message) here"),
        }
    }

    fn read(
        mut stream: TcpStream,
        stream_to_write: TcpStream,
        sender: Sender<Vec<u8>>,
    ) -> Result<(), Error> {
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
                Stream::close_streams(stream.try_clone()?, stream_to_write.try_clone()?)?;
                true
            }
        } {}
        Ok(())
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

    fn close_streams(stream_to_write: TcpStream, stream_to_read: TcpStream) -> Result<(), Error> {
        stream_to_read.shutdown(Shutdown::Both)?;
        stream_to_write.shutdown(Shutdown::Both)?;
        Ok(())
    }
}
