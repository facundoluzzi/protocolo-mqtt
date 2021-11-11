use crate::flags::connect_flags::ConnectFlags;
use crate::helper::remaining_length::save_remaining_length;
use crate::helper::status_code::ReturnCode;
use crate::helper::user_manager::UserManager;
use crate::payload::connect_payload::ConnectPayload;
use crate::topics::subscriber::Subscriber;

use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

pub struct Connect {
    _remaining_length: usize,
    flags: ConnectFlags,
    payload: ConnectPayload,
    status_code: ReturnCode,
}

impl Connect {
    pub fn init(bytes: &[u8], stream: &TcpStream, mut user_manager: UserManager) -> Connect {
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len).unwrap();

        let init_variable_header = 1 + readed_index;
        let end_variable_header = readed_index + 10;
        let variable_header = &bytes[init_variable_header..end_variable_header + 1];

        let connect_flags = ConnectFlags::init(&variable_header[7]);

        let payload = ConnectPayload::init(
            &connect_flags,
            &bytes[end_variable_header + 1..init_variable_header + remaining_length],
        );
        let flags = connect_flags;
        let status_code = match payload.check_authentication() {
            true => ReturnCode::Success,
            false => ReturnCode::NotAuthorized,
        };

        let connect = Connect {
            _remaining_length: remaining_length,
            flags,
            payload: payload,
            status_code,
        };

        if let Some(usuario) = user_manager.find_user(connect.get_client_id()) {
            usuario.assign_socket(stream);
        } else {
            let subscriber = connect.create_subscriber(stream);
            user_manager.add(subscriber);
        };

        connect
    }

    pub fn get_type(&self) -> String {
        "connect".to_owned()
    }

    pub fn send_response(&self, mut stream: &TcpStream) {
        let session_present_bit = !(0x01 & self.flags.get_clean_session_flag() as u8);
        let status_code = match self.status_code {
            ReturnCode::Success => 0x00,
            ReturnCode::NotAuthorized => 0x05,
        };
        let connack_response = [0x20, 0x02, session_present_bit, status_code];
        if let Err(msg_error) = stream.write(&connack_response) {
            println!("Error in sending response: {}", msg_error);
        }
    }

    pub fn create_subscriber(&self, socket: &TcpStream) -> Subscriber {
        Subscriber::new(self.payload.get_client_id(), socket.try_clone().unwrap())
    }

    pub fn get_client_id(&self) -> String {
        self.payload.get_client_id()
    }

    pub fn send_message(&self, _stream: &Sender<String>) {
        //todo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crear_paquete_connect_correctamente() {
        // Todos los vectores que se envian en los tests tienen desde la posición que finaliza la lectura del remaining length,
        // lo siguiente:
        // 6 bytes de protocol name 0x00u8 (length MSB(0)), 0x04u8 (length LSB(4)), 0x4Du8 (M), 0x15u8 (Q), 0x45u8 (T), 0x45u8 (T)
        // 1 byte de protocol level 0x04 que es lo que determina la versión del protocolo
        // 1 byte de content flag que representa que información puede haber en el payload
        // 2 bytes de keep alive
        // 0x0A -->  0 = 0000, A = 0110
        // el segundo byte indica el remaining length de largo 18, considerando el header variable, y 8 extras del payload: Client ID.
        // Se considera que los flags están vacíos en el índice 9, de otra manera habría que agregar tantos bytes como los flags indiquen
        // indice 9 -> byte 9 -> 0x00

        // let bytes = [
        //     0x10, 0x0E, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x04, 0x00, 0x00, 0x0B, 0x00, 0x02,
        //     0x00, 0x00,
        // ];

        // let connect_packet = Connect::init(&bytes);
        // assert_eq!(connect_packet.get_type(), "connect".to_owned());
    }
}
