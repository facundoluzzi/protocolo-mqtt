use crate::{packet_manager::ResponsePacket, trait_paquetes::Paquetes};

pub struct Default {}

impl Paquetes for Default {
    fn init(_bytes: &[u8]) -> Box<dyn Paquetes> {
        Box::new(Default {})
    }

    fn get_type(&self) -> ResponsePacket {
        ResponsePacket::Default
    }

    fn get_remaining_length(&self) -> usize {
        0
    }

    fn get_status_code(&self) -> u8 {
        0x00
    }
}
