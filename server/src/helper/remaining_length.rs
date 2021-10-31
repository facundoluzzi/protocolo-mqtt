/**
 * Se calcula la cantidad de bytes dentro del actual paquete, incluyendo la data del header variable
 * y el payload. Esto no incluye los bytes usados para encodear el remaining length.
 * Aclaraciones: toma los primeros 7 bits de cada byte porque el último está reservado como flag
 * para continuar procesando.
 * En cada byte procesado se multiplica el valor representado con esos 7 bits por 128^n siendo n
 * la posición del byte procesado.
 */
pub fn save_remaining_length(bytes: &[u8]) -> Result<(usize, usize), String> {
    let mut multiplier: usize = 1;
    let mut value: usize = 0;
    let mut index: usize = 0; // cantidad de bytes leídos
    loop {
        let encoded_byte: usize = bytes[index] as usize;
        value += (encoded_byte & 127) * multiplier;
        multiplier *= 128;
        if multiplier > (128 * 128 * 128) {
            // error
            return Err("".to_string());
        }
        if encoded_byte & 0b10000000 == 0 {
            return Ok((index + 1, value));
        }
        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obtener_remaining_length_12() {
        let bytes = [0x0C];

        let (readed_bytes, rem_length) = save_remaining_length(&bytes).unwrap();
        assert_eq!(readed_bytes, 1);
        assert_eq!(rem_length, 12);
    }

    #[test]
    fn obtener_remainin_length_127() {
        let bytes = [0x7F];
        let (readed_bytes, rem_length) = save_remaining_length(&bytes).unwrap();
        assert_eq!(readed_bytes, 1);
        assert_eq!(rem_length, 127);
    }

    #[test]
    fn obtener_remainin_length_128() {
        let bytes = [0x80, 0x01];
        let (readed_bytes, rem_length) = save_remaining_length(&bytes).unwrap();
        assert_eq!(readed_bytes, 2);
        assert_eq!(rem_length, 128);
    }

    #[test]
    fn obtener_remaining_length_129() {
        let bytes = [0x81, 0x01];
        let (readed_bytes, rem_length) = save_remaining_length(&bytes).unwrap();
        assert_eq!(readed_bytes, 2);
        assert_eq!(rem_length, 129);
    }

    #[test]
    fn obtener_remainin_length_129() {
        let bytes = [0xC1, 0x02];
        let (readed_bytes, rem_length) = save_remaining_length(&bytes).unwrap();
        assert_eq!(readed_bytes, 2);
        assert_eq!(rem_length, 321);
    }
}
