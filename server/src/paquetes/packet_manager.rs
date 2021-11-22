use crate::helper::user_manager::UserManager;
use crate::paquetes::connect::Connect;
use crate::paquetes::default::Default;
use crate::paquetes::publish::Publish;
use crate::paquetes::subscribe::Subscribe;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

use super::publisher_suscriber::PublisherSuscriber;

pub struct PacketManager {
    client_id: String,
}

impl PacketManager {
    pub fn init() -> Self {
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

    pub fn process_message(
        &mut self,
        bytes: &[u8],
        stream: &TcpStream,
        publisher_subscriber_sender: &Sender<PublisherSuscriber>,
        user_manager: UserManager,
    ) {
        let first_byte = bytes.get(0);

        match first_byte {
            Some(first_byte_ok) => match PacketManager::get_control_packet_type(*first_byte_ok) {
                1 => {
                    let connect = Connect::init(bytes, stream, user_manager);
                    self.set_client_id(connect.get_client_id());
                    connect.send_response(stream);
                }
                3 => Publish::init(bytes)
                    .send_message(publisher_subscriber_sender, self.client_id.to_owned())
                    .send_response(stream),
                8 => Subscribe::init(bytes)
                    .subscribe_topic(
                        publisher_subscriber_sender,
                        user_manager.get_sender(self.client_id.to_string()),
                        self.client_id.to_owned(),
                    )
                    .send_response(stream),
                _ => Default::init(bytes).send_response(stream),
            },
            None => Default::init(bytes).send_response(stream),
        };
    }
}
