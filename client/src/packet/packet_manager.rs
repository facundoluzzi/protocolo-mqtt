use crate::helper::remaining_length::save_remaining_length;
use crate::helper::utf8_parser::UTF8;
use crate::packet::input::connack::Connack;
use crate::packet::input::default;
use crate::packet::input::puback::Puback;
use crate::packet::input::suback::Suback;
use crate::packet::output::connack_response::ConnackResponse;
use crate::packet::output::default_response::DefaultResponse;
use crate::packet::output::puback_response::PubackResponse;
use crate::packet::output::publish_response::PublishResponse;
use crate::packet::output::suback_response::SubackResponse;
use crate::packet::output::unsuback_response::UnsubackResponse;
use crate::packet::sender_type::ClientSender;

use super::output::pingresp_response::PingrespResponse;

pub enum ResponsePacket {
    Connack,
    Suback,
    Puback,
    Publish,
    Default,
}

pub struct PacketManager {
    client_id: String,
}

impl Default for PacketManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketManager {
    pub fn new() -> Self {
        PacketManager {
            client_id: "".to_string(),
        }
    }

    pub fn get_control_packet_type(first_byte: u8) -> u8 {
        (0b11110000 & first_byte) >> 4
    }

    pub fn set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }

    pub fn process_publish(&self, bytes: &[u8]) -> Result<(String, String), String> {
        let qos_flag = (0x06 & bytes[0]) >> 1;
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, _remaining_length) = save_remaining_length(bytes_rem_len).unwrap();

        let init_variable_header = 1 + readed_index;

        let variable_header_response =
            match UTF8::utf8_parser(&bytes[init_variable_header..bytes.len()]) {
                Ok((parsed_topic, readed_bytes)) => {
                    let packet_identifier = &bytes[readed_bytes..readed_bytes + 2];
                    Ok((parsed_topic, packet_identifier, readed_bytes + 2))
                }
                Err(err) => Err(err),
            }?;

        let (topic, _packet_identifier, length) = variable_header_response;

        if qos_flag == 0x00 {
            let response = std::str::from_utf8(&bytes[init_variable_header + length..bytes.len()])
                .expect("err");
            Ok((topic, response.to_string()))
        } else {
            let response =
                std::str::from_utf8(&bytes[init_variable_header + 2 + length..bytes.len()])
                    .expect("err");
            Ok((topic, response.to_string()))
        }
    }

    pub fn process_message(&self, bytes: &[u8]) -> Option<ClientSender> {
        let first_byte = bytes.get(0);

        match first_byte {
            Some(first_byte_ok) => {
                let packet_type = PacketManager::get_control_packet_type(*first_byte_ok);
                println!("{}", packet_type);

                match packet_type {
                    2 => {
                        let connack = Connack::init(bytes);
                        let connack_code = connack.get_status_code();
                        let response_text = connack.status_for_code(connack_code);

                        let connack_response = ConnackResponse::init(response_text);

                        Some(ClientSender::Connack(connack_response))
                    }
                    3 => match self.process_publish(bytes) {
                        Ok((topic, message)) => {
                            let publish_response = PublishResponse::init(topic, message);
                            Some(ClientSender::Publish(publish_response))
                        }
                        Err(err) => {
                            println!("error: {}", err);
                            None
                        }
                    },
                    4 => {
                        let _puback = Puback::init(bytes);

                        let puback_response = PubackResponse::init("Publish realizado".to_string());
                        Some(ClientSender::Puback(puback_response))
                    }
                    9 => {
                        let suback = Suback::init(bytes);
                        let suback_codes = suback.get_status_code();
                        let response_text = suback.check_suback_code(suback_codes);

                        let suback_response = SubackResponse::init(response_text);

                        Some(ClientSender::Suback(suback_response))
                    }
                    11 => {
                        let unsuback_response =
                            UnsubackResponse::init("Unsubscribe realizado".to_string());
                        Some(ClientSender::Unsuback(unsuback_response))
                    }
                    14 => {
                        let pingresp_response = PingrespResponse::init();
                        Some(ClientSender::Pingresp(pingresp_response))
                    }
                    _ => {
                        default::Default::init(bytes);
                        let default_response =
                            DefaultResponse::init("Paquete no reconocido".to_string());
                        Some(ClientSender::Default(default_response))
                    }
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
