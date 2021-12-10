use crate::packet_manager::ResponsePacket;

pub struct Suback {
    remaining_length: u8,
    packet_identifier_msb: u8,
    packet_identifier_lsb: u8,
    status_codes: Vec<u8>,
}

impl Suback {
    fn get_remaining_length(&self) -> u8 {
        self.remaining_length
    }

    pub fn get_status_code(&self) -> &Vec<u8> {
        &self.status_codes
    }

    pub fn check_suback_code(&self, list_of_codes: &Vec<u8>) -> String {
        println!("ESTO ES LA LISTA DE CODIGOS: {:?}", list_of_codes);
        for code in list_of_codes {
            if *code != 0x00 && *code != 0x01 {
                return "Error en la suscripcion".to_string();
            }
        }
        "Suscripcion exitosa".to_string()
    }

    pub fn init(bytes: &[u8]) -> Suback {
        let remaining_length = bytes[1];
        let packet_identifier_msb = bytes[2];
        let packet_identifier_lsb = bytes[3];
        let list_of_qos = bytes[4..bytes.len()].to_vec();
        Suback {
            remaining_length: remaining_length,
            packet_identifier_msb,
            packet_identifier_lsb,
            status_codes: list_of_qos,
        }
    }

    fn get_type(&self) -> ResponsePacket {
        ResponsePacket::Suback
    }
}
