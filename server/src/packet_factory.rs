use crate::paquetes::Paquetes;
use crate::connect::Connect;
use crate::default::Default;

pub struct PacketFactory {}

impl PacketFactory {
    pub fn get(linea: String) -> Box<dyn Paquetes> {
        // paso el string a un array de bytes
        let bytes = linea.as_bytes();
        println!("{:?}", bytes);
        // obtengo el primer byte que tiene la configuracion
        let first_byte = *bytes.get(0).unwrap();
        // me quedo con los bits del paquete
        let mask = 0b11110000 & first_byte;
        match mask {
            0b00010000 => {
                Connect::new()
            }
            _ => {
                Default::new()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crear_paquete_connect_correctamente() {
        let bytes_packet = "\u{10}";
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