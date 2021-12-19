use crate::packet::output::trait_response::ResponseTrait;
pub struct PubackResponse {
    response: String,
}

impl ResponseTrait for PubackResponse {
    fn init(response: String) -> PubackResponse {
        PubackResponse { response }
    }

    /// Obtiene la respuesta en forma de String correspondiente al estado del paquete Puback recibido
    fn get_response(&self) -> String {
        self.response.to_string()
    }
}
