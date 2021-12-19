use crate::packet::output::trait_response::ResponseTrait;

pub struct DefaultResponse {
    response: String,
}

impl ResponseTrait for DefaultResponse {
    fn init(response: String) -> DefaultResponse {
        DefaultResponse { response }
    }

    fn get_response(&self) -> String {
        self.response.to_string()
    }
}
