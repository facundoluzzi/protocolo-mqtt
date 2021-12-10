use crate::connack::Connack;
use crate::default;
use crate::helper::remaining_length::save_remaining_length;
use crate::helper::utf8_parser::UTF8;
use crate::puback::Puback;
use crate::sender_types::connack_response::ConnackResponse;
use crate::sender_types::default_response::DefaultResponse;
use crate::sender_types::puback_response::PubackResponse;
use crate::sender_types::publish_response::PublishResponse;
use crate::sender_types::sender_type::ClientSender;
use crate::sender_types::suback_response::SubackResponse;
use crate::suback::Suback;

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

        println!("BYTES TOTALES: {:?}", bytes);

        println!(
            "BYTES DE MENSAJE: {:?}",
            &bytes[init_variable_header + length..bytes.len()]
        );

        if qos_flag == 0x00 {
            let response = std::str::from_utf8(&bytes[init_variable_header + length..bytes.len()])
                .expect("err");
            return Ok((topic, response.to_string()));
        } else {
            let response =
                std::str::from_utf8(&bytes[init_variable_header + 2 + length..bytes.len()])
                    .expect("err");
            return Ok((topic, response.to_string()));
        }
    }

    pub fn process_message(&self, bytes: &[u8]) -> Option<ClientSender> {
        println!("{:?}", &bytes);
        let first_byte = bytes.get(0);

        match first_byte {
            Some(first_byte_ok) => {
                let packet_type = PacketManager::get_control_packet_type(*first_byte_ok);

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

        // match packet_received.get_type() {
        //     ResponsePacket::Connack => {
        //         let response_code = packet_received.get_status_code();
        //         let text_response = packet_received.status_for_code(response_code);

        //         // // let (sender_to_get_message, receiver_to_get_message) = mpsc::channel::<Vec<u8>>();

        //         // // sender_stream.send((ReadStream, None, Some(sender_to_get_message))).unwrap();

        //         // // let message = receiver_to_get_message.recv().unwrap();

        //         // // receive_responses_from_broker(message);

        //         // let connack_code_received = rx.recv().unwrap();
        //         // let response = self.check_connack_code(connack_code_received);
        //         // let code = packet_received.get_status_code();
        //         // channel_producer.send(code).unwrap();
        //         Ok(())
        //     }
        //     ResponsePacket::Suback => {
        //         // let code = packet_received.get_status_code();
        //         // channel_producer.send(code).unwrap();
        //         Ok(())
        //     }
        //     ResponsePacket::Puback => {
        //         // let code = packet_received.get_status_code();
        //         // channel_producer.send(code).unwrap();
        //         Ok(())
        //     }
        //     // ResponsePacket::Publish => {
        //     //     // channel_producer.send(data).unwrap();
        //     // }
        //     _ => {
        //         println!("Received Default");
        //         Ok(())
        //     }
        // }
    }
}
