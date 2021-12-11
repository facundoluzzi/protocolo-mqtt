use crate::helper::flags::connack_flags::ConnackFlags;

pub struct Connack {
    _remaining_length: usize,
    _flags: ConnackFlags,
    status_code: u8,
}

impl Connack {
    /**
     * Se calcula la cantidad de bytes dentro del actual paquete, incluyendo la data del header variable
     * y el payload. Esto no incluye los bytes usados para encodear el remaining length.
     * Aclaraciones: toma los primeros 7 bits de cada byte porque el último está reservado como flag
     * para continuar procesando.
     * En cada byte procesado se multiplica el valor representado con esos 7 bits por 128^n siendo n
     * la posición del byte procesado.
     */

    pub fn get_status_code(&self) -> u8 {
        self.status_code
    }

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

    pub fn status_for_code(&self, code: u8) -> String {
        match code {
            0x00 => "Conexion realizada con exito".to_string(),
            0x01 => "Error: la version del protocolo no es compatible".to_string(),
            0x04 => "Error: los datos enviados no son correctos".to_string(),
            0x05 => "Error: no esta autorizado".to_string(),
            _ => "Error desconocido".to_string(),
        }
    }
}
