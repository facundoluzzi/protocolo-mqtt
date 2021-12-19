use crate::packet::output::trait_response::ResponseTrait;
pub struct PubackResponse {
    response: String,
}

impl ResponseTrait for PubackResponse {
    fn init(response: String) -> PubackResponse {
        PubackResponse { response }
    }

    fn get_response(&self) -> String {
        self.response.to_string()
    }
}
