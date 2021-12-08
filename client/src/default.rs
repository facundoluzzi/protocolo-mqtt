use crate::packet_manager::ResponsePacket;

pub struct Default {}

impl Default {
    pub fn init(_bytes: &[u8]) -> Default {
        Default {}
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
