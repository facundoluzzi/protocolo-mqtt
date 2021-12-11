pub struct PublishResponse {
    response: String,
    topic: String,
}

impl PublishResponse {
    pub fn init(topic: String, response: String) -> PublishResponse {
        PublishResponse { response, topic }
    }

    pub fn get_response(&self) -> String {
        self.response.to_string()
    }

    pub fn get_topic(&self) -> String {
        self.topic.to_string()
    }
}
