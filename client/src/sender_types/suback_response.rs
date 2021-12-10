pub struct SubackResponse {
    response: String,
}

impl SubackResponse {
    pub fn init(response: String) -> SubackResponse {
        SubackResponse { response }
    }

    pub fn get_response(&self) -> String {
        self.response.to_string()
    }
}
