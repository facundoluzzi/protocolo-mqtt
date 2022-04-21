use crate::packet::output::trait_response::ResponseTrait;

pub struct SubackResponse {
    response: String,
}

impl ResponseTrait for SubackResponse {
    fn init(response: String) -> SubackResponse {
        SubackResponse { response }
    }

    /// Obtiene la respuesta en forma de String correspondiente al estado del paquete Suback recibido
    fn get_response(&self) -> String {
        self.response.to_string()
    }
}
