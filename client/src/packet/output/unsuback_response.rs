use crate::packet::output::trait_response::ResponseTrait;

pub struct UnsubackResponse {
    response: String,
}

impl ResponseTrait for UnsubackResponse {
    fn init(response: String) -> UnsubackResponse {
        UnsubackResponse { response }
    }

    /// Obtiene la respuesta en forma de String correspondiente al estado del paquete Unsuback recibido
    fn get_response(&self) -> String {
        self.response.to_string()
    }
}
