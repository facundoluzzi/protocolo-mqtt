use crate::helper::remaining_length::save_remaining_length;
use crate::variable_header::publish_variable_header::get_variable_header;

use std::io::Read;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

pub struct Publish {
    _dup: u8,
    _qos: u8,
    _retain: u8,
    _remaining_length: usize,
    _topic: String,
    _packet_identifier: u8,
    _payload: String,
}

impl Publish {
    pub fn init(bytes: &[u8]) -> Publish {
        let dup_flag = 0x08 & bytes[0];
        let qos_flag = 0x06 & bytes[0];
        let retain_flag = 0x01 & bytes[0];

        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len).unwrap();

        let init_variable_header = 1 + readed_index;

        let (topic, packet_identifier, length) =
            get_variable_header(&bytes[init_variable_header..bytes.len()]);

        let payload = &bytes[init_variable_header + length..bytes.len()];

        Publish {
            _dup: dup_flag,
            _qos: qos_flag,
            _retain: retain_flag,
            _remaining_length: remaining_length,
            _topic: topic,
            _packet_identifier: packet_identifier[0],
            _payload: std::str::from_utf8(payload).unwrap().to_string(),
        }
    }

    pub fn get_type(&self) -> String {
        "publish".to_owned()
    }

    pub fn get_name(&self) -> String {
        self._topic
    }

    pub fn get_publish_message(&self) -> String {
        self._payload
    }

    pub fn send_response(&self, mut _stream: &TcpStream) {}

    pub fn send_message(&self, stream: &Sender<String>){
        //todo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crear_paquete_publish_correctamente() {
        // El primer byte está compuesto por el control packet header y por algunos flags.
        // El segundo byte es el remaining length que indica el largo del variable header mas el payload
        // el tercer y cuarto byte indican el largo del topic name, según el encoding msb o lsb, en este caso de 5 bytes
        // el byte nro 10 y 11 son los dos últimos bytes del variable header y son el packet identifier
        // los n siguientes bugs son parte del payload, en este caso vacío
        let bytes = [
            0x30, 0x0A, 0x00, 0x05, 0x54, 0x4F, 0x50, 0x49, 0x43, 0x00, 0x06, 0x54,
        ];
        let publish_packet = Publish::init(&bytes);
        assert_eq!(publish_packet.get_type(), "publish".to_owned());
    }
}
