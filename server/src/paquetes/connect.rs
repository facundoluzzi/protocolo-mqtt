use crate::flags::connect_flags::ConnectFlags;
use crate::helper::remaining_length::save_remaining_length;
use crate::paquetes::trait_paquetes::Paquetes;
use crate::payload::connect_payload::ConnectPayload;

use std::io::Write;
use std::net::TcpStream;

pub struct Connect {
    _remaining_length: usize,
    flags: ConnectFlags,
    _payload: ConnectPayload,
}

impl Paquetes for Connect {
    fn init(bytes: &[u8]) -> Box<dyn Paquetes> {
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
        Box::new(Connect {
            _remaining_length: remaining_length,
            flags,
            _payload: payload,
        })
    }

    fn get_type(&self) -> String {
        "connect".to_owned()
    }

    fn send_response(&self, mut stream: &TcpStream) {
        let session_present_bit = 0x01 & self.flags.get_clean_session_flag() as u8;
        let connack_response = [0x20, 0x02, session_present_bit, 0x00];
        if let Err(msg_error) = stream.write(&connack_response) {
            println!("Error in sending response: {}", msg_error);
        }
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

        let bytes = [
            0x10, 0x0E, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x04, 0x00, 0x00, 0x0B, 0x00, 0x02,
            0x00, 0x00,
        ];

        let connect_packet = Connect::init(&bytes);
        assert_eq!(connect_packet.get_type(), "connect".to_owned());
    }
}
