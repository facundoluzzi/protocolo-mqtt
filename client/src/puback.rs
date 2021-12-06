use crate::packet_manager::ResponsePacket;

pub struct Puback {
    remaining_length: usize,
    packet_identifier_msb: u8,
    packet_identifier_lsb: u8,
}

impl Puback {
    fn get_remaining_length(&self) -> usize {
        self.remaining_length
    }

    pub fn init(bytes: &[u8]) -> Puback {
        let variable_header = &bytes[2..4];
        let packet_identifier_msb = variable_header[0];
        let packet_identifier_lsb = variable_header[1];
        Puback {
            remaining_length: 2,
            packet_identifier_msb,
            packet_identifier_lsb,
        }
    }

    fn get_type(&self) -> ResponsePacket {
        ResponsePacket::Puback
    }
}
