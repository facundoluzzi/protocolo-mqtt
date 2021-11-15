use crate::connack::Connack;
use crate::default;
use crate::trait_paquetes::Paquetes;

pub struct PacketManager {
    client_id: String,
}

impl Default for PacketManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketManager {
    pub fn new() -> Self {
        PacketManager {
            client_id: "".to_string(),
        }
    }

    pub fn get_control_packet_type(first_byte: u8) -> u8 {
        (0b11110000 & first_byte) >> 4
    }

    pub fn set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }

    pub fn process_message(&self, bytes: &[u8]) -> Box<dyn Paquetes> {
        let first_byte = bytes.get(0);

        match first_byte {
            Some(first_byte_ok) => match PacketManager::get_control_packet_type(*first_byte_ok) {
                2 => Connack::init(bytes),
                _ => default::Default::init(bytes),
            },
            None => default::Default::init(bytes),
        }
    }
}
