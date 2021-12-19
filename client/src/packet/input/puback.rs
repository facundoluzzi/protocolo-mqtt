pub struct Puback {
    _remaining_length: usize,
    _packet_identifier_msb: u8,
    _packet_identifier_lsb: u8,
}

impl Puback {
    /// Inicializa un struct mediante un array de bytes representando al paquete Puback y sus diferentes propiedades
    pub fn init(bytes: &[u8]) -> Puback {
        let variable_header = &bytes[2..4];
        let _packet_identifier_msb = variable_header[0];
        let _packet_identifier_lsb = variable_header[1];
        Puback {
            _remaining_length: 2,
            _packet_identifier_msb,
            _packet_identifier_lsb,
        }
    }
}
