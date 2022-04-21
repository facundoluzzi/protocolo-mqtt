use crate::packet::output::trait_response::ResponseTrait;

pub struct ConnackResponse {
    response: String,
}

impl ResponseTrait for ConnackResponse {
    fn init(response: String) -> ConnackResponse {
        ConnackResponse { response }
    }

    /// Obtiene la respuesta en forma de String correspondiente al estado del paquete connack recibido
    fn get_response(&self) -> String {
        self.response.to_string()
    }
}
