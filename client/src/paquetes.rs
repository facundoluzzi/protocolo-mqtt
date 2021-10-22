pub trait PACKET {
    fn get_type(&self) -> String;
}

#[derive(Debug)]
pub enum PACKETS {
    CONNECT,
    CONNACK,
    DEFAULT,
}

impl PACKET for PACKETS {
    fn get_type(&self) -> String{
        match *self {
            PACKETS::CONNECT => "connect".to_owned(),
            PACKETS::CONNACK => "connack".to_owned(),
            PACKETS::DEFAULT => "default".to_owned()
        }
    }
}

pub struct CONNECT {}

pub struct CONNACK {}

pub struct DEFAULT {}

impl CONNECT {
    pub fn get_type() -> String {
        "connect".to_owned()   
    }
}

impl CONNACK {
    pub fn get_type() -> String {
        "CONNACK".to_owned()   
    }
}

impl DEFAULT {
    pub fn get_type() -> String {
        "DEFAULT".to_owned()   
    }
}
pub struct PACKETFACTORY {}

impl PACKETFACTORY {
    pub fn get(packet: &[u8]) -> PACKETS {
        match packet {
            [0b0010000u8] => PACKETS::CONNECT,
            [0b0100000u8] => PACKETS::CONNACK,
            _ => PACKETS::DEFAULT
        }
    }
}



pub struct PacketBuilder {}

impl PacketBuilder  {
    pub fn new(linea: String) -> PACKETS {
        println!("linea: {}", linea);
        // paso el string a un array de bytes
        let bytes = linea.as_bytes();
        println!("bytes: {:?}", bytes);

        // obtengo el primer byte que tiene la configuracion
        let first_byte = *bytes.get(0).unwrap();
        println!("first_byte: {}", first_byte);

        // me quedo con los bits del paquete
        let mask = 0b11110000 & first_byte;
        println!("mask: {}", mask);
        match mask {
            0b00010000 => {
                PACKETS::CONNECT{}
            }
            0b00100000 => {
                PACKETS::CONNACK{}
            }
            _ => {
                PACKETS::DEFAULT
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crear_paquete_connect_correctamente() {
        let bytes_packet: u8 = 0b00010000;
        let paquete_creado = PacketBuilder::new(bytes_packet.to_string());
        println!("{:?}", paquete_creado);
        // assert_eq!(paquete_creado.get_type(), CONNECT{});
    }
}
