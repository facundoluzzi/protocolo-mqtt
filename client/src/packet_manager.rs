use crate::connack::Connack;
use crate::default;
use crate::puback::Puback;
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

    pub fn process_message(&self, bytes: &[u8]) -> Option<(ResponsePacket, String)> {
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
                        Some((ResponsePacket::Connack, response_text))
                    }
                    3 => {
                        // cuando el paquete publish tiene qos 1, hay que hacer la confirmación al server
                        Some((ResponsePacket::Publish, "TODO".to_string()))
                    }
                    4 => {
                        println!("\n\n\n llega el puback \n\n\n");
                        let _puback = Puback::init(bytes);
                        Some((ResponsePacket::Puback, "Publish realizado".to_string()))
                    }
                    9 => {
                        println!("\n\n\n suback recibido \n\n\n");
                        let suback = Suback::init(bytes);
                        let suback_code = suback.get_status_code();
                        let response_text = suback.check_suback_code(suback_code);
                        Some((ResponsePacket::Suback, response_text))
                    }
                    _ => {
                        default::Default::init(bytes);
                        Some((
                            ResponsePacket::Default,
                            "paquete no identificado".to_string(),
                        ))
                    }
                }
            }
            None => {
                default::Default::init(bytes);
                Some((
                    ResponsePacket::Default,
                    "paquete no identificado".to_string(),
                ))
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
