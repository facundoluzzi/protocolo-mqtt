use crate::packet::output::trait_response::ResponseTrait;

pub struct DisconnectResponse {
    response: String,
}

impl ResponseTrait for DisconnectResponse {
    fn init(response: String) -> Self {
        DisconnectResponse { response }
    }

    /// Obtiene la respuesta en forma de String correspondiente al estado del paquete Disconnect recibido
    fn get_response(&self) -> String {
        self.response.to_string()
    }
}
