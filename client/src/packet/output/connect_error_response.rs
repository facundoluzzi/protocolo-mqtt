pub struct ConnectErrorResponse {
    response: String,
}

impl ConnectErrorResponse {
    pub fn init(response: String) -> ConnectErrorResponse {
        ConnectErrorResponse { response }
    }

    pub fn get_response(&self) -> String {
        self.response.to_string()
    }
}
