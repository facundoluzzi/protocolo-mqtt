pub struct DisconnectResponse {}

impl DisconnectResponse {
    pub fn init() -> DisconnectResponse {
        DisconnectResponse {}
    }

    pub fn get_response(&self) -> String {
        "Desconexión exitosa".to_string()
    }
}
