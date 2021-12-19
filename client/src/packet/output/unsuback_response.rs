use crate::packet::output::trait_response::ResponseTrait;

pub struct UnsubackResponse {
    response: String,
}

impl ResponseTrait for UnsubackResponse {
    fn init(response: String) -> UnsubackResponse {
        UnsubackResponse { response }
    }

    fn get_response(&self) -> String {
        self.response.to_string()
    }
}
