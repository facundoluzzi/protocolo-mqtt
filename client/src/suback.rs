use crate::{packet_manager::ResponsePacket, trait_paquetes::Paquetes};

pub struct Suback {
    remaining_length: usize,
    packet_identifier_msb: u8,
    packet_identifier_lsb: u8,
    status_code: u8,
}

impl Paquetes for Suback {
    fn get_remaining_length(&self) -> usize {
        self.remaining_length
    }

    fn get_status_code(&self) -> u8 {
        self.status_code
    }

    fn init(bytes: &[u8]) -> Box<dyn Paquetes> {
        let variable_header = &bytes[2..5];
        let packet_identifier_msb = variable_header[0];
        let packet_identifier_lsb = variable_header[1];
        let status_code = variable_header[2];
        Box::new(Suback {
            remaining_length: 2,
            packet_identifier_msb,
            packet_identifier_lsb,
            status_code,
        })
    }

    fn get_type(&self) -> ResponsePacket {
        ResponsePacket::Suback
    }
}
