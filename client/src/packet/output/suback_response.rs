use crate::packet::output::trait_response::ResponseTrait;

pub struct SubackResponse {
    response: String,
}

impl ResponseTrait for SubackResponse {
    fn init(response: String) -> SubackResponse {
        SubackResponse { response }
    }

    fn get_response(&self) -> String {
        self.response.to_string()
    }
}
