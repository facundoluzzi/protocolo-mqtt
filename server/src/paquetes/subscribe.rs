use crate::helper::remaining_length::save_remaining_length;
use crate::paquetes::trait_paquetes::Paquetes;

use std::io::Write;
use std::net::TcpStream;

pub struct Subscribe {
    _remaining_length: usize,
    _packet_identifier: u8,
}

impl Paquetes for Subscribe {
    fn init(bytes: &[u8]) -> Box<dyn Paquetes> {
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len).unwrap();

        let init_variable_header = 1 + readed_index;
        let end_variable_header = init_variable_header + 2;

        let _payload = &bytes[end_variable_header..bytes.len()];

        Box::new(Subscribe {
            _remaining_length: remaining_length,
            _packet_identifier: bytes[init_variable_header],
        })
    }

    fn get_type(&self) -> String {
        "subscribe".to_owned()
    }

    fn send_response(&self, mut stream: &TcpStream) {
        let bytes_response = [0x90, 0x03, 0x00, 0x00, 0x00];
        if let Err(msg_error) = stream.write(&bytes_response) {
            println!("Error in sending response: {}", msg_error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crear_paquete_subscribe_correctamente() {
        // El primer byte está compuesto por el control packet header y por algunos flags.
        // El segundo byte es el remaining length que indica el largo del variable header mas el payload
        // el tercer y cuarto byte indican el largo del topic name, según el encoding msb o lsb, en este caso de 5 bytes
        // el byte nro 10 y 11 son los dos últimos bytes del variable header y son el packet identifier
        // los n siguientes bugs son parte del payload, en este caso vacío
        let bytes = [
            0x30, 0x01, 0x00, 0x05, 0x54, 0x4F, 0x50, 0x49, 0x43, 0x00, 0x06,
        ];
        let publish_packet = Subscribe::init(&bytes);
        assert_eq!(publish_packet.get_type(), "subscribe".to_owned());
    }
}
