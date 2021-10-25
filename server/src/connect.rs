use crate::paquetes::Paquetes;
use crate::utf8_parser::UTF8;
pub struct Connect {
    remaining_length: usize,
    flags: ConnectFlags,
    payload: ConnectPayload,
}

pub struct ConnectPayload {
    client_identifier: u8,
    will_topic: Option<u8>,
    will_message: Option<u8>,
    username: Option<u8>,
    password: Option<u8>,
}

impl ConnectPayload {
    fn new(connectFlags: ConnectFlags, remaining_bytes: &[u8]) -> ConnectPayload {
        let index: usize = 0;
        let pointer: usize = 0;
        let (client_identifier, index) = UTF8::utf8_parser(&remaining_bytes[pointer + 1..remaining_bytes.len()]);                
        let (will_topic, index) = if connectFlags.get_will_flag() {UTF8::utf8_parser(&remaining_bytes[pointer + 1..remaining_bytes.len()])} else {(None, 0};
        pointer += index;
        let (will_message, index) = if connectFlags.get_will_flag() {UTF8::utf8_parser(&remaining_bytes[pointer + 1..remaining_bytes.len()])} else {(None, 0)};
        pointer += index;
        let (username, index) = if connectFlags.get_username_flag() {UTF8::utf8_parser(&remaining_bytes[pointer + 1..remaining_bytes.len()])} else {(None, 0)};
        pointer += index;
        let (password, index) = if connectFlags.get_password_flag() & connectFlags.get_username_flag() {UTF8::utf8_parser(&remaining_bytes[pointer + 1..remaining_bytes.len()])} else {(None, 0)};

        ConnectPayload {
            client_identifier: client_identifier,
            username: username,
            password: password,
            will_topic: will_topic,
            will_message: will_message,
        }
    }
}

pub struct ConnectFlags {
    username: bool,
    password: bool,
    will_retain: bool,
    will_qos: u8,
    will_flag: bool,
    clean_session: bool,
}

impl ConnectFlags {
    fn new(bytes: &u8) -> ConnectFlags {
        ConnectFlags {
            will_qos: (0b00011000 & bytes) >> 3,
            username: 0b10000000 & bytes != 0,
            password: 0b01000000 & bytes != 0,
            will_retain: 0b00100000 & bytes != 0,
            will_flag: 0b00000100 & bytes != 0,
            clean_session: 0b00000010 & bytes != 0,
        }
    }

    fn get_username_flag(&self) -> bool {
        self.username
    }
    fn get_password_flag(&self) -> bool {
        self.password
    }
    fn get_will_retain_flag(&self) -> bool {
        self.will_retain
    }
    fn get_will_flag(&self) -> bool {
        self.will_flag
    }
    fn get_clean_session_flag(&self) -> bool {
        self.clean_session
    }
    fn get_will_qos_flag(&self) -> u8 {
        self.will_qos
    }
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
        let mut index: usize = 0;
        loop {
            let encoded_byte: usize = bytes[index] as usize;
            value += (encoded_byte & 127) * multiplier;
            multiplier *= 128;
            if multiplier > (128 * 128 * 128) {
                // error
                return Err("".to_string());
            }
            self.remaining_length = value;
            if encoded_byte & 128 == 0 {
                return Ok(index + 1);
            }
            index += 1;
        }
    }

    fn get_remaining_length(&self) -> usize {
        self.remaining_length
    }

    fn init(bytes: &[u8]) -> Box<dyn Paquetes> {
        let flag_null = ConnectFlags::new(&0x00u8);
        let payload_null = ConnectPayload::new(flag_null, &[0x00u8]);
        let mut packet = Box::new(Connect {
            remaining_length: 0,
            flags: flag_null,
            payload: payload_null,
        });

        let index = packet
            .save_remaining_length(&bytes[1..bytes.len()])
            .unwrap();
        let variable_header = &bytes[index..index + 10];
        let connect_flags = ConnectFlags::new(&variable_header[8]);
        packet.flags = connect_flags;

        //let payload = &bytes[index + 10..packet.get_remaining_length()];

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
        // todos los vectores que se envian en los tests tienen desde la posición que finaliza la lectura del remaining length,
        // lo siguiente:
        // 6 bytes de protocol name 0x00u8 (length MSB(0)), 0x04u8 (length LSB(4)), 0x4Du8 (M), 0x15u8 (Q), 0x45u8 (T), 0x45u8 (T)
        // 1 byte de protocol level 0x04 que es lo que determina la versión del protocolo
        // 1 byte de content flag que representa que información puede haber en el payload
        // 2 bytes de keep alive

        // 64 en decimal
        let first_bytes = [
            0b00010000u8,
            0x40u8,
            0x00u8,
            0x04u8,
            0x4Du8,
            0x15u8,
            0x45u8,
            0x45u8,
            0x04u8,
            0xFFu8,
            0x00u8,
            0x0Au8,
        ];
        let first_connect_packet = Connect::init(&first_bytes);
        assert_eq!(first_connect_packet.get_remaining_length(), 64);

        // representar el 127 en decimal
        let second_bytes = [
            0b00010000u8,
            0x7Fu8,
            0x00u8,
            0x04u8,
            0x4Du8,
            0x15u8,
            0x45u8,
            0x45u8,
            0x04u8,
            0xFFu8,
            0x00u8,
            0x0Au8,
        ];
        let second_connect_packet = Connect::init(&second_bytes);
        assert_eq!(second_connect_packet.get_remaining_length(), 127);

        // representar el 128 en decimal
        let third_bytes = [
            0b00010000u8,
            0x80u8,
            0x01u8,
            0x00u8,
            0x04u8,
            0x4Du8,
            0x15u8,
            0x45u8,
            0x45u8,
            0x04u8,
            0xFFu8,
            0x00u8,
            0x0Au8,
        ];
        let third = Connect::init(&third_bytes);
        assert_eq!(third.get_remaining_length(), 128);

        // representar el 129 en decimal
        let bytes = [
            0b00010000u8,
            0x81u8,
            0x01u8,
            0x00u8,
            0x04u8,
            0x4Du8,
            0x15u8,
            0x45u8,
            0x45u8,
            0x04u8,
            0xFFu8,
            0x00u8,
            0x0Au8,
        ];
        let third = Connect::init(&bytes);
        assert_eq!(third.get_remaining_length(), 129);

        // representar el 321 en decimal
        let bytes = [
            0b00010000u8,
            0xC1u8,
            0x02u8,
            0x01u8,
            0x00u8,
            0x04u8,
            0x4Du8,
            0x15u8,
            0x45u8,
            0x45u8,
            0x04u8,
            0xFFu8,
            0x00u8,
            0x0Au8,
        ];
        let third = Connect::init(&bytes);
        assert_eq!(third.get_remaining_length(), 321);
    }
}
