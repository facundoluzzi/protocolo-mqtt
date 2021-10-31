use crate::flags::connect_flags::ConnectFlags;
use crate::flags::trait_flags::Flags;
use crate::paquetes::Paquetes;
use crate::payload::connect_payload::ConnectPayload;
use crate::payload::trait_payload::Payload;

use std::io::Write;
use std::net::TcpStream;

pub struct Connack {
    remaining_length: usize,
    flags: Box<dyn Flags>,
    payload: Box<dyn Payload>,
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
    fn save_remaining_length(&mut self, bytes: &[u8]) -> Result<usize, String> {
        let mut multiplier: usize = 1;
        let mut value: usize = 0;
        let mut index: usize = 0; // cantidad de bytes leídos
        loop {
            let encoded_byte: usize = bytes[index] as usize;
            value += (encoded_byte & 127) * multiplier;
            multiplier *= 128;
            if multiplier > (128 * 128 * 128) {
                // error
                return Err("".to_string());
            }
            self.remaining_length = value;
            if encoded_byte & 0b10000000 == 0 {
                return Ok(index + 1);
            }
            index += 1;
        }
    }

    fn get_remaining_length(&self) -> usize {
        self.remaining_length
    }

    fn init(bytes: &[u8]) -> Box<dyn Paquetes> {
        let flag_null = ConnackFlags::init(&0x00u8);
        let mut packet = Box::new(Connack {
            remaining_length: 0,
            flags: flag_null,
        });
        let readed_index = packet
            .save_remaining_length(&bytes[1..bytes.len()])
            .unwrap();
        let init_variable_header = 1 + readed_index;
        let end_variable_header = readed_index + 2;
        let variable_header = &bytes[init_variable_header..end_variable_header + 1];
        let connack_flags = ConnackFlags::init(&variable_header[7]);

        packet.flags = connect_flags;
        packet
    }

    fn get_type(&self) -> String {
        "connack".to_owned()
    }

    fn get_payload(&self) -> &dyn Payload {
        self.payload.as_ref()
    }

    fn send_response(&self, mut stream: &TcpStream) {
        if let Err(msg_error) = stream.write("connack\n".as_bytes()) {
            println!("Error in sending response: {}", msg_error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obtener_remaining_length_correctamente() {
        let first_bytes = [
            0x20, // Paquete CONNACK
            0x02, // Remaining Length - 2 para paquete CONNACK, ya que no tiene payload
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x04, 0x00, 0x00, 0x0B, 0x00, 0x06,
            0x41, 0x4A, 0x0C, 0x0B, 0x5C, 0x42,
        ];

        //let first_connect_packet = Connect::init(&first_bytes);
        //assert_eq!(first_connect_packet.get_remaining_length(), 18);
}
