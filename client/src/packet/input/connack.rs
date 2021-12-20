use crate::helper::flags::connack_flags::ConnackFlags;

pub struct Connack {
    _remaining_length: usize,
    _flags: ConnackFlags,
    status_code: u8,
}

impl Connack {
    /// Devuelve el status code que esta presente en el paquete Connack
    pub fn get_status_code(&self) -> u8 {
        self.status_code
    }

    /// Inicializa un struct mediante un array de bytes representando al paquete connack y sus diferentes propiedades
    pub fn init(bytes: &[u8]) -> Connack {
        let variable_header = &bytes[2..4];
        let connack_flags = ConnackFlags::init(&variable_header[0]);
        let connack_code = variable_header[1];
        Connack {
            _remaining_length: 2,
            _flags: connack_flags,
            status_code: connack_code,
        }
    }

    /// Devuelve un texto a mostrar a la interfaz segun cada codigo de Connack recibido
    pub fn status_for_code(&self, code: u8) -> String {
        match code {
            0x00 => "Conexion realizada con exito".to_string(),
            0x01 => "Error: la version del protocolo no es compatible".to_string(),
            0x02 => "Error: el ID de cliente no esta disponible".to_string(),
            0x04 => "Error: los datos enviados no son correctos".to_string(),
            0x05 => "Error: no esta autorizado".to_string(),
            _ => "Error desconocido".to_string(),
        }
    }
}
