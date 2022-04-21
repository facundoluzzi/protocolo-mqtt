use crate::packet::output::trait_response::ResponseTrait;
pub struct ConnectErrorResponse {
    response: String,
}

impl ResponseTrait for ConnectErrorResponse {
    fn init(response: String) -> ConnectErrorResponse {
        ConnectErrorResponse { response }
    }

    /// Obtiene la respuesta en forma de String correspondiente al estado de error que se recibio desde el connack recibido
    fn get_response(&self) -> String {
        self.response.to_string()
    }
}
