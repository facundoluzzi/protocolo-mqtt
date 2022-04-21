use crate::helper::remaining_length::save_remaining_length;
use crate::types::StreamType;
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
    StopTimeout,
}

impl Stream {
    ///Inicializa una estructura que representa un TCPStream, crea channels que van a ser utilizados
    /// para ver la accion a realizar con respecto al TCPStream, estritura o lectura.
    /// Devuelve el sender de dicho stream o un error en caso de fallar al clonar
    pub fn init(stream: TcpStream) -> Result<Sender<StreamType>, std::io::Error> {
        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) =
            mpsc::channel();

        if let Err(err) = stream.set_read_timeout(Some(Duration::from_secs(5))) {
            println!("Unexpected error setting first timeout: {}", err);
        }

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

    /// Procesa la accion de escritura que va a ser realizada en el stream
    fn process_write_action(
        message: Option<Vec<u8>>,
        stream_to_write: TcpStream,
    ) -> Result<(), std::io::Error> {
        if let Some(msg) = message {
            Stream::write(stream_to_write, msg);
            Ok(())
        } else {
            panic!("Unexpected error: send a Some(message) here");
        }
    }

    /// Procesa la accion de lectura que va a ser realizada en el stream
    fn process_read_action(
        sender: Option<Sender<Vec<u8>>>,
        stream_to_write: TcpStream,
        stream_to_read: TcpStream,
    ) -> Result<(), std::io::Error> {
        if let Some(sender) = sender {
            thread::spawn(move || {
                Stream::read(stream_to_read, stream_to_write, sender);
            });
            Ok(())
        } else {
            panic!("Unexpected error: send a Some(Sender<String>) here");
        }
    }

    /// Procesa la accion de desconexion que va a ser realizada en el stream
    fn process_close_connection_action(
        stream_to_read: TcpStream,
        stream_to_write: TcpStream,
    ) -> Result<(), std::io::Error> {
        if let Err(_err_msg) = stream_to_read.shutdown(Shutdown::Both) {
            return Err(_err_msg);
        }

        if let Err(_err_msg) = stream_to_write.shutdown(Shutdown::Both) {
            return Err(_err_msg);
        }
        Ok(())
    }

    /// Procesa la accion de eliminar el timeout que va a ser realizada en el stream
    fn process_stop_timeout_action(
        stream_to_write: TcpStream,
        stream_to_read: TcpStream,
    ) -> Result<(), std::io::Error> {
        if let Err(err) = stream_to_read.set_read_timeout(None) {
            println!("Unexpected error setting keep alive: {}", err);
        }

        if let Err(err) = stream_to_write.set_read_timeout(None) {
            println!("Unexpected error setting keep alive: {}", err);
        }
        Ok(())
    }

    /// Hace un match de las dos acciones para realizar por el stream, escritura o lectura.
    /// En cada caso lee o escribe segun lo recibido o corta la conexion con el server en caso que
    /// llegue una accion que represente a un error.
    fn match_action_for_stream(
        message_received: StreamType,
        stream_to_write: TcpStream,
        stream_to_read: TcpStream,
    ) -> Result<(), std::io::Error> {
        let action = message_received.0;
        match action {
            StreamAction::WriteStream => {
                Stream::process_write_action(message_received.1, stream_to_write)
            }
            StreamAction::ReadStream => {
                Stream::process_read_action(message_received.2, stream_to_write, stream_to_read)
            }
            StreamAction::CloseConnectionStream => {
                Stream::process_close_connection_action(stream_to_read, stream_to_write)
            }
            StreamAction::StopTimeout => {
                Stream::process_stop_timeout_action(stream_to_write, stream_to_read)
            }
        }
    }

    /// Escribe un mensaje en el TCPStream
    fn write(mut stream: TcpStream, message: Vec<u8>) {
        let c: &[u8] = &message;
        if let Err(msg_error) = stream.write(c) {
            println!("{}", msg_error);
        }
    }

    /// Procesa los primeros bytes del paquete recibido, identificando las diferentes partes necesarias para seguir
    /// procesando dicho paquete
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

    /// Lee el paquete recibido y lo va haciendo de a 5 bytes hasta llegar al final del paquete, luego de eso lo procesa
    /// cuando la lectura se da por concluida
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
                Stream::finish_to_process_packet(
                    &mut total_data,
                    packet_length,
                    readed_bytes,
                    &mut is_first_byte,
                    sender.clone(),
                    size,
                )
            }
            Err(_err) => {
                if let Err(_err_msg) = stream.shutdown(Shutdown::Both) {}
                if let Err(_err_msg) = stream_to_write.shutdown(Shutdown::Both) {}
                true
            }
        } {}
    }

    /// Una vez leido y almacenado todos los bytes del paquete recibido, los procesa y arma para poder ser enviado
    fn finish_to_process_packet(
        total_data: &mut Vec<u8>,
        packet_length: usize,
        readed_bytes: usize,
        is_first_byte: &mut bool,
        sender: Sender<Vec<u8>>,
        size: usize,
    ) -> bool {
        if total_data.len() > packet_length + readed_bytes {
            *is_first_byte = true;
            let bytes_to_process = &total_data[0..packet_length + readed_bytes + 1];
            if sender.send(bytes_to_process.to_vec()).is_err() {
                return false;
            }

            *total_data = Vec::new();
            false
        } else if *is_first_byte && size == 0 {
            if sender.send(vec![]).is_err() {
                return false;
            }
            false
        } else {
            true
        }
    }
}
