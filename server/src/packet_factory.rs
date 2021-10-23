use crate::connect::Connect;
use crate::default::Default;
use crate::paquetes::Paquetes;

pub struct PacketFactory {}

impl PacketFactory {
    fn get_control_packet_type(first_byte: u8) -> u8 {
        (0b11110000 & first_byte) >> 4
    }

    pub fn get(linea: String) -> Box<dyn Paquetes> {
        let bytes = linea.as_bytes();
        let first_byte = bytes.get(0);

        match first_byte {
            Some(first_byte_ok) => match PacketFactory::get_control_packet_type(*first_byte_ok) {
                1 => Connect::init(bytes),
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
        let bytes_packet = "\u{10}\u{40}";
        let prueba = PacketFactory::get(bytes_packet.to_owned());
        assert_eq!(prueba.get_type(), "connect".to_owned());
    }

    #[test]
    fn crear_paquete_default() {
        let bytes_packet = "\u{00}";
        let prueba = PacketFactory::get(bytes_packet.to_owned());
        assert_eq!(prueba.get_type(), "default".to_owned());
    }
}
