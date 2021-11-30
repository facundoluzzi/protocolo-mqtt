pub fn get_variable_header(bytes: &[u8]) ->  Result<(Vec<u8>, usize), String>{
    if bytes.len() < 3 {
        return Err("Length should be at least 3".to_string());
    }

    let msb = bytes[0];
    let lsb = bytes[1];

    let encoding_length = bytes.len() - 2;
    if !(encoding_length >= (msb as usize) && encoding_length >= (lsb as usize)) {
        return Err("Malformed code".to_string());
    }

    // Length es el largo total del campo que estamos parseando: username, password etc
    let length: usize;
    let init: usize;
    let end: usize;
    let mut copy_bytes = bytes.to_vec();
    if msb > lsb {
        copy_bytes.reverse();
        init = 0;
        end = usize::from(msb);
        length = usize::from(msb + 0b00000010);
    } else {
        init = 2;
        end = usize::from(lsb) + 0b00000010;
        length = usize::from(lsb + 0b00000010);
    }

    Ok((
        (copy_bytes[init..end].to_vec()),
        length,
    ))
}


pub fn verify_wilcard(topic: String) -> bool {
    for i in 0..topic.len() {
        let b: u8 = topic.as_bytes()[i];
        let c: char = b as char;
        if c == '/' && i < topic.len() {
            let b: u8 = topic.as_bytes()[i + 1];
            let c: char = b as char;
            if c == '#' && (i + 2) != topic.len() {
                return false;
            }
        }
    }
    true
}
