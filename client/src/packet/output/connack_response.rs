pub struct ConnackResponse {
    response: String,
}

impl ConnackResponse {
    pub fn init(response: String) -> ConnackResponse {
        ConnackResponse { response }
    }

    pub fn get_response(&self) -> String {
        self.response.to_string()
    }
}
