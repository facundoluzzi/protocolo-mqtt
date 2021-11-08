pub struct PublisherSuscriber {
    code: i32,
    topic: String,
    message: Option<String>,
}

impl PublisherSuscriber {
    pub fn new(topic: String, message: String, code: i32) -> PublisherSuscriber{
        PublisherSuscriber{
            topic, message, code,
        }
    }
    pub fn get_packet_type(self) -> String {
        match code {
            0 => "Publish",
            1 => "Suscriber",
            _ => "None",
        }
    }
    pub fn get_topic(&self) -> String {
        self.topic
    }

    // pub fn get_message(&self) -> String {
    //     self.message
    // }

}