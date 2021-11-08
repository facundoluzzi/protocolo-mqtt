#[derive(Debug)]
pub struct Subscriber {}

impl Clone for Subscriber {
    fn clone(&self) -> Self {
        Subscriber {}
    }
}

impl Subscriber {
    pub fn new(bytes: &[u8]) -> Subscriber {
        // let dup_flag = 0x08 & bytes[0];
        // let qos_flag = 0x06 & bytes[0];
        // let retain_flag = 0x01 & bytes[0];

        // let bytes_rem_len = &bytes[1..bytes.len()];
        // let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len).unwrap();

        // let init_variable_header = 1 + readed_index;

        // let (topic, packet_identifier, length) =
        //     get_variable_header(&bytes[init_variable_header..bytes.len()]);

        // let payload = &bytes[init_variable_header + length..bytes.len()];

        // Subscriber {
        //     _dup: dup_flag,
        //     _qos: qos_flag,
        //     _retain: retain_flag,
        //     _remaining_length: remaining_length,
        //     _topic: topic,
        //     _packet_identifier: packet_identifier[0],
        //     _payload: std::str::from_utf8(payload).unwrap().to_string(),
        // }

        Subscriber {}
    }
    pub fn publish_message(&self, _message: String) {}

    // pub fn get_topics(&self) -> String {
    //     self.payload
    // }
}
