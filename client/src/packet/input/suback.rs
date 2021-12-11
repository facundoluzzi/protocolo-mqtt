pub struct Suback {
    _remaining_length: u8,
    _packet_identifier_msb: u8,
    _packet_identifier_lsb: u8,
    status_codes: Vec<u8>,
}

impl Suback {
    pub fn get_status_code(&self) -> &Vec<u8> {
        &self.status_codes
    }

    pub fn check_suback_code(&self, list_of_codes: &[u8]) -> String {
        for code in list_of_codes {
            if *code != 0x00 && *code != 0x01 {
                return "Error en la suscripcion".to_string();
            }
        }
        "Suscripcion exitosa".to_string()
    }

    pub fn init(bytes: &[u8]) -> Suback {
        let _remaining_length = bytes[1];
        let _packet_identifier_msb = bytes[2];
        let _packet_identifier_lsb = bytes[3];
        let list_of_qos = bytes[4..bytes.len()].to_vec();
        Suback {
            _remaining_length,
            _packet_identifier_msb,
            _packet_identifier_lsb,
            status_codes: list_of_qos,
        }
    }
}
