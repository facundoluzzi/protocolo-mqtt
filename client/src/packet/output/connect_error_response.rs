use crate::packet::output::trait_response::ResponseTrait;
pub struct ConnectErrorResponse {
    response: String,
}

impl ResponseTrait for ConnectErrorResponse {
    fn init(response: String) -> ConnectErrorResponse {
        ConnectErrorResponse { response }
    }

    fn get_response(&self) -> String {
        self.response.to_string()
    }
}
