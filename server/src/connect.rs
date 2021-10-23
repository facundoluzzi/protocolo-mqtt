use crate::paquetes::Paquetes;

pub struct Connect {
    remaining_length: usize,
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
    fn save_remaining_length(&mut self, bytes: &[u8]) -> Result<String, String> {
        let mut multiplier: usize = 1;
        let mut value: usize = 0;
        let mut index: usize = 0;
        loop {
            let encoded_byte: usize = bytes[index] as usize;
            value += (encoded_byte & 127) * multiplier;
            multiplier *= 128;
            if multiplier > (128 ^ 3) {
                // error
                return Err("".to_string());
            }
            self.remaining_length = value;
            if encoded_byte & 128 == 0 {
                return Ok("ok".to_string());
            }
            index += 1;
        }
    }

    fn get_remaining_length(&self) -> usize {
        self.remaining_length
    }

    fn init(bytes: &[u8]) -> Box<dyn Paquetes> {
        let mut packet = Box::new(Connect {
            remaining_length: 0,
        });
        println!("{:?}", bytes);
        packet
            .save_remaining_length(&bytes[1..bytes.len()])
            .unwrap();
        packet
    }

    fn get_type(&self) -> String {
        "connect".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obtener_remaining_length_correctamente() {
        // 64 en decimal
        let bytes = [0b00010000u8, 0b01000000u8];
        let first_connect_packet = Connect::init(&bytes);
        assert_eq!(first_connect_packet.get_remaining_length(), 64);

        // representar el 127 en decimal
        let bytes = [0b00010000u8, 0x7Fu8];
        let second_connect_packet = Connect::init(&bytes);
        assert_eq!(second_connect_packet.get_remaining_length(), 127);

        // representar el 128 en decimal
        let bytes = [0b00010000u8, 0x80u8, 0x01u8];
        let third = Connect::init(&bytes);
        assert_eq!(third.get_remaining_length(), 128);

        // representar el 129 en decimal
        let bytes = [0b00010000u8, 0x81u8, 0x01u8];
        let third = Connect::init(&bytes);
        assert_eq!(third.get_remaining_length(), 129);

        // representar el 321 en decimal
        let bytes = [0b00010000u8, 0xC1u8, 0x02u8, 0x01u8];
        let third = Connect::init(&bytes);
        assert_eq!(third.get_remaining_length(), 321);
    }
}
