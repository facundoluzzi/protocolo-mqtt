use crate::{packet_manager::ResponsePacket, trait_paquetes::Paquetes};

pub struct Puback {
    remaining_length: usize,
    packet_identifier_msb: u8,
    packet_identifier_lsb: u8,
}

impl Paquetes for Puback {
    fn get_remaining_length(&self) -> usize {
        self.remaining_length
    }

    fn get_status_code(&self) -> u8 {
        0
    }

    fn init(bytes: &[u8]) -> Box<dyn Paquetes> {
        let variable_header = &bytes[2..4];
        let packet_identifier_msb = variable_header[0];
        let packet_identifier_lsb = variable_header[1];
        Box::new(Puback {
            remaining_length: 2,
            packet_identifier_msb,
            packet_identifier_lsb,
        })
    }

    fn get_type(&self) -> ResponsePacket {
        ResponsePacket::Puback
    }
}
