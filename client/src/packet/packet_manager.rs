use std::sync::mpsc::Sender;

use crate::helper::remaining_length::save_remaining_length;
use crate::helper::stream::stream_handler::StreamAction;
use crate::helper::variable_header::get_variable_header;
use crate::packet::input::connack::Connack;
use crate::packet::input::default;
use crate::packet::input::puback::Puback;
use crate::packet::input::suback::Suback;
use crate::packet::output::connack_response::ConnackResponse;
use crate::packet::output::default_response::DefaultResponse;
use crate::packet::output::puback_response::PubackResponse;
use crate::packet::output::publish_response::PublishResponse;
use crate::packet::output::suback_response::SubackResponse;
use crate::packet::output::trait_response::ResponseTrait;
use crate::packet::output::unsuback_response::UnsubackResponse;
use crate::packet::sender_type::ClientSender;
use crate::types::{PublishPacket, StreamType};

use super::output::pingresp_response::PingrespResponse;

pub enum ResponsePacket {
    Connack,
    Suback,
    Puback,
    Publish,
    Default,
}

pub struct PacketManager {}

impl Default for PacketManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketManager {
    pub fn new() -> Self {
        PacketManager {}
    }

    /// Obtiene el control packet type de un paquete
    pub fn get_control_packet_type(first_byte: u8) -> u8 {
        (0b11110000 & first_byte) >> 4
    }

    /// Procesa el paquete publish que es mandando desde el broker a un cliente que es suscriptor
    /// obtiene el topico, mensaje, qos y packet ID y lo devuelve
    pub fn process_publish(&self, bytes: &[u8]) -> PublishPacket {
        let qos_flag = (0x06 & bytes[0]) >> 1;
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, _remaining_length) = save_remaining_length(bytes_rem_len)?;

        let init_variable_header = 1 + readed_index;
        let variable_header = &bytes[init_variable_header..bytes.len()];
        let (topic, packet_identifier, length) = get_variable_header(variable_header, qos_flag)?;

        let response =
            std::str::from_utf8(&bytes[init_variable_header + length..bytes.len()]).expect("err");
        Ok((topic, response.to_string(), qos_flag, packet_identifier))
    }

    /// Procesa un paquete Connack recibido desde el server y envia la respuesta a la interfaz para imprimir el exito o error del paquete
    fn process_connack_packet(
        &self,
        bytes: &[u8],
        sender_stream: Sender<StreamType>,
    ) -> Option<ClientSender> {
        let connack = Connack::init(bytes);
        let connack_code = connack.get_status_code();
        let response_text = connack.status_for_code(connack_code);
        if let Err(err) = sender_stream.send((StreamAction::StopTimeout, None, None)) {
            println!("unexpected error: {}", err);
        }
        let connack_response = ConnackResponse::init(response_text);

        Some(ClientSender::Connack(connack_response))
    }

    /// Procesa un paquete Publish recibido desde el server y envia la respuesta a la interfaz para imprimir el exito o error del paquete
    fn process_publish_packet(&self, bytes: &[u8]) -> Option<ClientSender> {
        match self.process_publish(bytes) {
            Ok((topic, message, qos, packet_identifier)) => {
                let publish_response =
                    PublishResponse::init(topic, message, qos, packet_identifier);
                Some(ClientSender::Publish(publish_response))
            }
            Err(err) => {
                println!("error: {}", err);
                None
            }
        }
    }

    /// Procesa un paquete Puback recibido desde el server y envia la respuesta a la interfaz para imprimir el exito o error del paquete
    fn process_puback_packet(&self, bytes: &[u8]) -> Option<ClientSender> {
        let _puback = Puback::init(bytes);

        let puback_response = PubackResponse::init("Publish realizado".to_string());
        Some(ClientSender::Puback(puback_response))
    }

    /// Procesa un paquete Suback recibido desde el server y envia la respuesta a la interfaz para imprimir el exito o error del paquete
    fn process_suback_packet(&self, bytes: &[u8]) -> Option<ClientSender> {
        let suback = Suback::init(bytes);
        let suback_codes = suback.get_status_code();
        let response_text = suback.check_suback_code(suback_codes);

        let suback_response = SubackResponse::init(response_text);

        Some(ClientSender::Suback(suback_response))
    }

    /// Procesa un paquete Unsuback recibido desde el server y envia la respuesta a la interfaz para imprimir el exito o error del paquete
    fn process_unsuback_packet(&self) -> Option<ClientSender> {
        let unsuback_response = UnsubackResponse::init("Unsubscribe realizado".to_string());
        Some(ClientSender::Unsuback(unsuback_response))
    }

    /// Procesa un paquete Pingresp recibido desde el server y envia la respuesta a la interfaz para imprimir el exito o error del paquete
    fn process_pingresp_packet(&self) -> Option<ClientSender> {
        let pingresp_response = PingrespResponse::init();
        Some(ClientSender::Pingresp(pingresp_response))
    }

    /// Procesa un paquete Default recibido desde el server y envia la respuesta a la interfaz para imprimir el exito o error del paquete
    fn process_default_packet(&self, bytes: &[u8]) -> Option<ClientSender> {
        default::Default::init(bytes);
        let default_response = DefaultResponse::init("Paquete no reconocido".to_string());
        Some(ClientSender::Default(default_response))
    }

    /// Hace un match del packet type para iniciar el proceso de un paquete mandado desde el broker hacia el cliente
    pub fn process_message(
        &self,
        bytes: &[u8],
        sender_stream: Sender<StreamType>,
    ) -> Option<ClientSender> {
        let first_byte = bytes.get(0);

        match first_byte {
            Some(first_byte_ok) => {
                let packet_type = PacketManager::get_control_packet_type(*first_byte_ok);

                match packet_type {
                    2 => self.process_connack_packet(bytes, sender_stream),
                    3 => self.process_publish_packet(bytes),
                    4 => self.process_puback_packet(bytes),
                    9 => self.process_suback_packet(bytes),
                    11 => self.process_unsuback_packet(),
                    14 => self.process_pingresp_packet(),
                    _ => self.process_default_packet(bytes),
                }
            }
            None => {
                default::Default::init(bytes);
                let default_response = DefaultResponse::init("Paquete no reconocido".to_string());
                Some(ClientSender::Default(default_response))
            }
        }
    }
}
