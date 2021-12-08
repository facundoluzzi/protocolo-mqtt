pub struct PublishMessage {
    message: Vec<u8>,
    qos: u8,
    retained_message: bool,
}

impl PublishMessage {
    pub fn init(message: Vec<u8>, qos: u8, retained_message: bool) -> PublishMessage {
        PublishMessage {
            message,
            qos,
            retained_message,
        }
    }

    pub fn get_message(&self) -> Vec<u8> {
        self.message.clone()
    }

    pub fn get_qos(&self) -> u8 {
        self.qos
    }

    pub fn get_retained_message(&self) -> bool {
        self.retained_message
    }
}
