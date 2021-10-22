struct Connect {}
struct Default {}

pub trait Paquetes {
    fn new() -> Box<dyn Paquetes> where Self: Sized;
    fn get_type(&self) -> String;
}

// Implement the `Animal` trait for `Sheep`.
impl Paquetes for Connect {
    fn new() -> Box<dyn Paquetes> {
        Box::new(Connect {})
    }
    fn get_type(&self) -> String {
        "connect".to_owned()
    }
}

// Implement the `Animal` trait for `Cow`.
impl Paquetes for Default {
    fn new() -> Box<dyn Paquetes> {
        Box::new(Default {})
    }
    fn get_type(&self) -> String {
        "default".to_owned()
    }
}


pub struct PacketFactory {}

impl PacketFactory {
    pub fn get(linea: String) -> Box<dyn Paquetes> {
        // paso el string a un array de bytes
        let bytes = linea.as_bytes();
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
        let bytes_packet = "\u{1f}";
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

// pub struct PUBLISH {}

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