pub struct DefaultResponse {
    response: String,
}

impl DefaultResponse {
    pub fn init(response: String) -> DefaultResponse {
        DefaultResponse { response }
    }

    pub fn get_response(&self) -> String {
        self.response.to_string()
    }
}
