pub struct PublishToStream {
    message: Vec<u8>,
}

impl PublishToStream {
    pub fn init(message: Vec<u8>) -> Self {
        PublishToStream { message }
    }

    pub fn get_message(&self) -> Vec<u8> {
        self.message.clone()
    }
}
