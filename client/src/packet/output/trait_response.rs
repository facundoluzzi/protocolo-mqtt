///Trait que representa a las respuestas mandadas a la interfaz segun que paquete se recibio en el cliente.
pub trait ResponseTrait {
    fn init(response: String) -> Self
    where
        Self: Sized;
    fn get_response(&self) -> String;
}
