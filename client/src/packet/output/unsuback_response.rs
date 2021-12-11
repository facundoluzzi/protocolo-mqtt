pub struct UnsubackResponse {
    response: String,
}

impl UnsubackResponse {
    pub fn init(response: String) -> UnsubackResponse {
        UnsubackResponse { response }
    }

    pub fn get_response(&self) -> String {
        self.response.to_string()
    }
}
