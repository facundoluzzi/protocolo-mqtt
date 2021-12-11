pub struct PublishMessage {
    all_bytes: Vec<u8>,
    qos: u8,
    retained_message: bool,
    message: String,
}

impl PublishMessage {
    pub fn init(
        all_bytes: Vec<u8>,
        qos: u8,
        retained_message: bool,
        message: String,
    ) -> PublishMessage {
        PublishMessage {
            all_bytes,
            qos,
            retained_message,
            message,
        }
    }

    pub fn get_packet(&self) -> Vec<u8> {
        self.all_bytes.clone()
    }

    pub fn get_message(&self) -> String {
        self.message.to_string()
    }

    pub fn get_qos(&self) -> u8 {
        self.qos
    }

    pub fn get_retained_message(&self) -> bool {
        self.retained_message
    }
}
