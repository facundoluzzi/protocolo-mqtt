pub struct PubackResponse {
    response: String,
}

impl PubackResponse {
    pub fn init(response: String) -> PubackResponse {
        PubackResponse { response }
    }

    pub fn get_response(&self) -> String {
        self.response.to_string()
    }
}
