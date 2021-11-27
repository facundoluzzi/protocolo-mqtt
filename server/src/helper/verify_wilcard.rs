pub fn verify_wilcard(topic: String) -> Option<String> {
    let mut string_to_return = "".to_owned();
    for i in 0..topic.len() {
        let b: u8 = topic.as_bytes()[i];
        let c: char = b as char;
        if c == '/' && i < topic.len() {
            let c: u8 = topic.as_bytes()[i + 1];
            let d: char = c as char;
            if d == '#' && (i + 1) == (topic.len() - 1) {
                return Some(string_to_return);
            }
        }
        string_to_return.push(c);
    }
    None
}