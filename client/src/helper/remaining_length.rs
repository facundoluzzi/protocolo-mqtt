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
