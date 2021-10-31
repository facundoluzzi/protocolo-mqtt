use crate::paquetes::subscribe::Subscribe;
use crate::paquetes::connect::Connect;
use crate::paquetes::default::Default;
use crate::paquetes::trait_paquetes::Paquetes;
use crate::paquetes::publish::Publish;

pub struct PacketFactory {}

impl PacketFactory {
    fn get_control_packet_type(first_byte: u8) -> u8 {
        (0b11110000 & first_byte) >> 4
    }

    pub fn get(bytes: &[u8]) -> Box<dyn Paquetes> {
        let first_byte = bytes.get(0);

        match first_byte {
            Some(first_byte_ok) => match PacketFactory::get_control_packet_type(*first_byte_ok) {
                1 => Connect::init(bytes),
                3 => Publish::init(bytes),
                8 => Subscribe::init(bytes),
                _ => Default::init(bytes),
            },
            None => Default::init(bytes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crear_paquete_connect_correctamente() {
        let first_bytes = [
            0x10, 0x0C, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x04, 0x00, 0x00, 0x0B, 0x01, 0x02,
        ];
        let connect_packet = PacketFactory::get(&first_bytes);
        assert_eq!(connect_packet.get_type(), "connect".to_owned());
    }

    #[test]
    fn crear_paquete_publish_correctamente() {
        let first_bytes = [0x30];
        let publish_packet = PacketFactory::get(&first_bytes);
        assert_eq!(publish_packet.get_type(), "publish".to_owned());
    }

    #[test]
    fn crear_paquete_default() {
        let bytes_packet = [
            0x00, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x04, 0xFF, 0x00, 0x0A, 0x00, 0x06, 0x50,
            0x52, 0x55, 0x45, 0x42, 0x41,
        ];
        let prueba = PacketFactory::get(&bytes_packet);
        assert_eq!(prueba.get_type(), "default".to_owned());
    }
}
