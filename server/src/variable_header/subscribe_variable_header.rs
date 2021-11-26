pub fn get_variable_header(bytes: &[u8]) -> (&[u8], usize) {
    let packet_identifier = &bytes[0..2];
    (packet_identifier, 2)
}


pub fn verify_wilcard(topic: String) -> bool {
    for i in 0..topic.len() {
        let b: u8 = topic.as_bytes()[i];
        let c: char = b as char; 
        if c == '/' && i < topic.len() {
            let b: u8 = topic.as_bytes()[i + 1];
            let c: char = b as char; 
            if c == '#' && (i+2) != topic.len() {
                return false;
            }
        }
    }
    true 
}