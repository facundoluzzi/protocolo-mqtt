use crate::flags::connack_flags::ConnackFlags;
use crate::paquetes::Paquetes;
use client::return_connack::get_code;

use std::io::Write;
use std::net::TcpStream;

pub struct Connack {
    remaining_length: usize,
    _flags: ConnackFlags,
    _return_code: String,
}

impl Paquetes for Connack {
    /**
     * Se calcula la cantidad de bytes dentro del actual paquete, incluyendo la data del header variable
     * y el payload. Esto no incluye los bytes usados para encodear el remaining length.
     * Aclaraciones: toma los primeros 7 bits de cada byte porque el último está reservado como flag
     * para continuar procesando.
     * En cada byte procesado se multiplica el valor representado con esos 7 bits por 128^n siendo n
     * la posición del byte procesado.
     */
    fn save_remaining_length(&mut self, _bytes: &[u8]) -> Result<usize, String> {
        Ok(2)
    }

    fn get_remaining_length(&self) -> usize {
        self.remaining_length
    }

    fn init(bytes: &[u8]) -> Box<dyn Paquetes> {
        let variable_header = &bytes[2..4];
        let connack_flags = ConnackFlags::init(&variable_header[0]);
        let connack_code = get_code(variable_header[1]);
        Box::new(Connack {
            remaining_length: 2,
            _flags: connack_flags,
            _return_code: connack_code,
        })
    }

    fn get_type(&self) -> String {
        "connack".to_owned()
    }
    fn send_response(&self, mut stream: &TcpStream) {
        if let Err(msg_error) = stream.write("connack\n".as_bytes()) {
            println!("Error in sending response: {}", msg_error);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{connack::Connack, paquetes::Paquetes};

    #[test]
    fn create_connack() {
        let first_bytes = [
            0x20, // Paquete CONNACK
            0x02, // Remaining Length - 2 para paquete CONNACK, ya que no tiene payload
            0x01, 0x00,
        ];

        let first_connack_packet = Connack::init(&first_bytes);
        assert_eq!(first_connack_packet.get_remaining_length(), 2)
    }
}
