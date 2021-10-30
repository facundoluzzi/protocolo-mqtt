use crate::flags::connect_flags::ConnectFlags;
use crate::flags::trait_flags::Flags;
use crate::paquetes::Paquetes;
use crate::payload::connect_payload::ConnectPayload;
use crate::payload::trait_payload::Payload;

use std::io::Write;
use std::net::TcpStream;

pub struct Connect {
    remaining_length: usize,
    flags: Box<dyn Flags>,
    payload: Box<dyn Payload>,
}

impl Paquetes for Connect {
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
        let flag_null = ConnectFlags::init(&0x00u8);
        let payload_null = ConnectPayload::init(&flag_null, &[0x00u8]);
        let mut packet = Box::new(Connect {
            remaining_length: 0,
            flags: flag_null,
            payload: payload_null,
        });

        let readed_index = packet
            .save_remaining_length(&bytes[1..bytes.len()])
            .unwrap();

        let init_variable_header = 1 + readed_index;
        let end_variable_header = readed_index + 10;
        let variable_header = &bytes[init_variable_header..end_variable_header + 1];
        let connect_flags = ConnectFlags::init(&variable_header[7]);
        packet.payload = ConnectPayload::init(
            &connect_flags,
            &bytes[end_variable_header + 1..init_variable_header + packet.get_remaining_length()],
        );
        packet.flags = connect_flags;
        packet
    }

    fn get_type(&self) -> String {
        "connect".to_owned()
    }

    fn get_payload(&self) -> &Box<dyn Payload> {
        &self.payload
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

        let first_bytes = [
            0x10, 0x12, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x04, 0x00, 0x00, 0x0B, 0x00, 0x06,
            0x41, 0x4A, 0x0C, 0x0B, 0x5C, 0x42,
        ];

        let first_connect_packet = Connect::init(&first_bytes);
        assert_eq!(first_connect_packet.get_remaining_length(), 18);

        // // representar el 127 en decimal
        // let second_bytes = [
        //     0b00010000u8,
        //     0x7Fu8,
        //     0x00u8,
        //     0x04u8,
        //     0x4Du8,
        //     0x15u8,
        //     0x45u8,
        //     0x45u8,
        //     0x04u8,
        //     0xFFu8,
        //     0x00u8,
        //     0x0Au8,
        // ];
        // let second_connect_packet = Connect::init(&second_bytes);
        // assert_eq!(second_connect_packet.get_remaining_length(), 127);

        // // representar el 128 en decimal
        // let third_bytes = [
        //     0b00010000u8,
        //     0x80u8,
        //     0x01u8,
        //     0x00u8,
        //     0x04u8,
        //     0x4Du8,
        //     0x15u8,
        //     0x45u8,
        //     0x45u8,
        //     0x04u8,
        //     0xFFu8,
        //     0x00u8,
        //     0x0Au8,
        // ];
        // let third = Connect::init(&third_bytes);
        // assert_eq!(third.get_remaining_length(), 128);

        // // representar el 129 en decimal
        // let bytes = [
        //     0b00010000u8,
        //     0x81u8,
        //     0x01u8,
        //     0x00u8,
        //     0x04u8,
        //     0x4Du8,
        //     0x15u8,
        //     0x45u8,
        //     0x45u8,
        //     0x04u8,
        //     0xFFu8,
        //     0x00u8,
        //     0x0Au8,
        // ];
        // let third = Connect::init(&bytes);
        // assert_eq!(third.get_remaining_length(), 129);

        // // representar el 321 en decimal
        // let bytes = [
        //     0b00010000u8,
        //     0xC1u8,
        //     0x02u8,
        //     0x01u8,
        //     0x00u8,
        //     0x04u8,
        //     0x4Du8,
        //     0x15u8,
        //     0x45u8,
        //     0x45u8,
        //     0x04u8,
        //     0xFFu8,
        //     0x00u8,
        //     0x0Au8,
        // ];
        // let third = Connect::init(&bytes);
        // assert_eq!(third.get_remaining_length(), 321);
    }

    // #[test]
    // fn leer_payload_correctamente() {
    //     let bytes = [
    //         0x10,
    //         0x80,
    //         0x01,
    //         0x00,
    //         0x04,
    //         0x4D,
    //         0x15,
    //         0x45,
    //         0x45,
    //         0x04,
    //         0x00,
    //         0x00,
    //         0x0A,
    //         0x00, // empieza el payload
    //         0x06,
    //         0x50,
    //         0x52,
    //         0x55,
    //         0x45,
    //         0x42,
    //         0x41
    //     ];
    //     let connect_packet = Connect::init(&bytes);

    // }
}
