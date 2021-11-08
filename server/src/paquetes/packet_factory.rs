use std::sync::mpsc::Sender;
use crate::paquetes::connect::Connect;
use crate::paquetes::default::Default;
use crate::paquetes::publish::Publish;
use crate::paquetes::subscribe::Subscribe;

pub struct PacketFactory {
    client_id: String
}

impl PacketFactory {
    pub fn new() -> Self {
        PacketFactory {
            client_id: "".to_string()
        }
    }
    
    pub fn get_control_packet_type(first_byte: u8) -> u8 {
        (0b11110000 & first_byte) >> 4
    }

    pub fn set_client_id(&self, client_id: String) {
        self.client_id = client_id;
    }

    pub fn process_message(self, bytes: &[u8], stream: &Sender<String>) {
        let first_byte = bytes.get(0);

        match first_byte {
            Some(first_byte_ok) => match PacketFactory::get_control_packet_type(*first_byte_ok) {
                1 => Connect::init(bytes).send_message(stream),
                3 => Publish::init(bytes).send_message(stream),
                8 => Subscribe::init(bytes).send_message(stream),
                _ => Default::init(bytes).send_message(stream),
            },
            None => Default::init(bytes).send_message(stream),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    // fn crear_paquete_connect_correctamente() {
    //     let bytes = [
    //         0x10, 0x0E, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x04, 0x00, 0x00, 0x0B, 0x00, 0x02,
    //         0x00, 0x00,
    //     ];
    //     let connect_packet = PacketFactory::get(&bytes);
    //     assert_eq!(connect_packet.get_type(), "connect".to_owned());
    // }

    // #[test]
    // fn crear_paquete_publish_correctamente() {
    //     let bytes = [
    //         0x30, 0x0A, 0x00, 0x05, 0x54, 0x4F, 0x50, 0x49, 0x43, 0x00, 0x06, 0x54,
    //     ];
    //     let publish_packet = PacketFactory::get(&bytes);
    //     assert_eq!(publish_packet.get_type(), "publish".to_owned());
    // }

    // #[test]
    // fn crear_paquete_default() {
    //     let bytes_packet = [
    //         0x00, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x04, 0xFF, 0x00, 0x0A, 0x00, 0x06, 0x50,
    //         0x52, 0x55, 0x45, 0x42, 0x41,
    //     ];
    //     let prueba = PacketFactory::get(&bytes_packet);
    //     assert_eq!(prueba.get_type(), "default".to_owned());
    // }
}
