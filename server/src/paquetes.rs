pub trait Paquetes {
    fn get_type(&self) -> String;
}

pub struct Connect;
impl Paquetes for Connect {
    #[inline(never)]
    fn get_type(&self) -> String {
        "connect".to_string()
    }
}

// pub struct CONNAC {}

// impl CONNAC {
//     pub fn get_type() -> String {
//         "connac".to_owned()   
//     }
// }

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
    pub fn new(linea: String) -> Connect {
        let bytes = linea.as_bytes();
        let first_byte = *bytes.get(0).unwrap();
        let mask = 0b11110000 & first_byte;
        match mask {
            0b00010000 => {
                Connect{}
            }
            _ => {
                Connect{}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crear_paquete_connect_correctamente() {
        let connect_packet = PacketBuilder::new("\u{10}".to_owned());
        assert_eq!(connect_packet.get_type(), "connect".to_string());
    }
}
