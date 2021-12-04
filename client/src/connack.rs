use crate::flags::connack_flags::ConnackFlags;
use crate::packet_manager::ResponsePacket;
use crate::trait_paquetes::Paquetes;
pub struct Connack {
    remaining_length: usize,
    _flags: ConnackFlags,
    status_code: u8,
}

impl Paquetes for Connack {
    /**
     * Se calcula la cantidad de bytes dentro del actual paquete, incluyendo la data del header variable
     * y el payload. Esto no incluye los bytes usados para encodear el remaining length.
     * Aclaraciones: toma los primeros 7 bits de cada byte porque el último está reservado como flag
     * para continuar procesando.
     * En cada byte procesado se multiplica el valor representado con esos 7 bits por 128^n siendo n
     * la posición del byte procesado.
     */

    fn get_remaining_length(&self) -> usize {
        self.remaining_length
    }

    fn get_status_code(&self) -> u8 {
        self.status_code
    }

    fn init(bytes: &[u8]) -> Box<dyn Paquetes> {
        let variable_header = &bytes[2..4];
        let connack_flags = ConnackFlags::init(&variable_header[0]);
        let connack_code = variable_header[1];
        Box::new(Connack {
            remaining_length: 2,
            _flags: connack_flags,
            status_code: connack_code,
        })
    }

    fn get_type(&self) -> ResponsePacket {
        ResponsePacket::Connack
    }
}
