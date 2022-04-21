use crate::packet::output::trait_response::ResponseTrait;

pub struct DefaultResponse {
    response: String,
}

impl ResponseTrait for DefaultResponse {
    fn init(response: String) -> DefaultResponse {
        DefaultResponse { response }
    }

    /// Obtiene la respuesta en forma de String correspondiente al paquete default, que es un paquete creado de un codigo desconocido
    fn get_response(&self) -> String {
        self.response.to_string()
    }
}
