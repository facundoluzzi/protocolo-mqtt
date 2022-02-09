pub struct UTF8 {}

impl UTF8 {
    /// Parsea una serie de bytes para transformarlos en un string, para ello toma en cuenta
    /// los diferentes bytes que contiene cada paquete para calcular la longitud correcta y poder
    /// desencodearlo de manera adecuada para todos los paquetes
    pub fn utf8_parser(bytes: &[u8]) -> Result<(String, usize), String> {
        let copy_bytes = bytes.to_vec();
        if bytes.len() < 3 {
            return Err("Length should be at least 3".to_string());
        }
        let msb = bytes[0];
        let lsb = bytes[1];
        let encoding_length = bytes.len() - 2;
        if !(encoding_length >= (msb as usize) && encoding_length >= (lsb as usize)) {
            return Err("Malformed code".to_string());
        }

        let (init, end, length, copy_bytes) =
            UTF8::calculate_init_end_and_length_of_variable_header(msb, lsb, copy_bytes);
        match String::from_utf8(copy_bytes[init..end].to_vec()) {
            Ok(result) => Ok((result, length)),
            Err(_) => Err("Could not convert ut8 to string".to_string()),
        }
    }

    /// Calcula la longitud, el inici y final del variable header del paquete que tiene los bytes que se buscan parsear
    pub fn calculate_init_end_and_length_of_variable_header(
        msb: u8,
        lsb: u8,
        mut copy_bytes: Vec<u8>,
    ) -> (usize, usize, usize, Vec<u8>) {
        if msb > lsb {
            let init = 0;
            let end = usize::from(msb);
            copy_bytes = copy_bytes[2..end + 2].to_vec();
            copy_bytes.reverse();
            let length = usize::from(msb + 0b00000010);
            (init, end, length, copy_bytes)
        } else {
            let init = 2;
            let end = usize::from(lsb) + 0b00000010;
            let length = usize::from(lsb + 0b00000010);
            (init, end, length, copy_bytes)
        }
    }
}
