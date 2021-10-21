#[derive(Debug)]
pub enum PAQUETES {
    CONNECT,
    CONNAC,
    DEFAULT
}

pub struct CONNECT {}

pub struct CONNAC {}

impl CONNECT {
    pub fn get_type() -> String {
        "connect".to_owned()   
    }
}

impl CONNAC {
    pub fn get_type() -> String {
        "connac".to_owned()   
    }
}


//pub struct PUBLISH {}

// pub struct PUBACK {}

// pub struct PUBREL {}

// pub struct PUBCOMP {}

// pub struct SUBSCRIBE {}

// pub struct SUBACK {}

// pub struct UNSUSCRIBE {}

// pub struct UNSUBACK {}

// pub struct PINGREQ {}

// pub struct PINGRESP {}

// pub struct DISCONNECT {}

pub struct PacketBuilder {}

impl PacketBuilder  {
    pub fn new(linea: String) -> PAQUETES {
        println!("linea: {:?}", linea);
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
                PAQUETES::CONNECT{}
            }
            0b00100000 => {
                PAQUETES::CONNAC{}
            }
            _ => {
                PAQUETES::DEFAULT
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crear_paquete_connect_correctamente() {
        let paquete_creado = PacketBuilder::new("\u{1f}".to_owned());
        println!("{:?}", paquete_creado);
        // assert_eq!(paquete_creado.get_type(), CONNECT{});q
    }
}
