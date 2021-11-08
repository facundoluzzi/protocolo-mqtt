pub struct PublisherSuscriber {
    code: i32,
    topic: String,
    message: String,
}

impl PublisherSuscriber {
    pub fn new(topic: String, message: String, code: i32) -> PublisherSuscriber{
        PublisherSuscriber{
            topic,
            message,
            code,
        }
    }
    pub fn get_packet_type(&self) -> String {
        match self.code {
            0 => "Publish".to_owned(),
            1 => "Suscriber".to_owned(),
            _ => "None".to_owned(),
        }
    }
    pub fn get_topic(&self) -> String {
        self.topic.to_owned()
    }

    pub fn get_message(&self) -> String {
        self.message.to_owned()
    }

}