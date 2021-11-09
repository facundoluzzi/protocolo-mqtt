use crate::paquetes::connect::Connect;
use crate::paquetes::default::Default;
use crate::paquetes::publish::Publish;
use crate::paquetes::subscribe::Subscribe;
use crate::user_manager::UserManager;
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
        &self,
        bytes: &[u8],
        stream: &TcpStream,
        publisher_subscriber_sender: &Sender<PublisherSuscriber>,
        user_manager: UserManager,
    ) {
        let first_byte = bytes.get(0);

        match first_byte {
            Some(first_byte_ok) => match PacketManager::get_control_packet_type(*first_byte_ok) {
                1 => {
                    let connect = Connect::init(bytes);
                    if let Some(usuario) = user_manager.find_user(connect.get_client_id()) {
                        usuario.assign_socket(stream);
                    }else {
                        let subscriber = connect.create_subscriber(stream);
                        user_manager.add(subscriber);
                    }
                },
                3 => Publish::init(bytes).send_message(stream, publisher_subscriber_sender),
                8 => Subscribe::init(bytes)
                    .subscribe_topic(publisher_subscriber_sender)
                    .send_response(stream),
                _ => Default::init(bytes).send_response(stream),
            },
            None => Default::init(bytes).send_response(stream),
        };
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    //#[test]
    // fn crear_paquete_connect_correctamente() {
    //     let bytes = [
    //         0x10, 0x0E, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x04, 0x00, 0x00, 0x0B, 0x00, 0x02,
    //         0x00, 0x00,
    //     ];
    //     let connect_packet = PacketManager::get(&bytes);
    //     assert_eq!(connect_packet.get_type(), "connect".to_owned());
    // }

    // #[test]
    // fn crear_paquete_publish_correctamente() {
    //     let bytes = [
    //         0x30, 0x0A, 0x00, 0x05, 0x54, 0x4F, 0x50, 0x49, 0x43, 0x00, 0x06, 0x54,
    //     ];
    //     let publish_packet = PacketManager::get(&bytes);
    //     assert_eq!(publish_packet.get_type(), "publish".to_owned());
    // }

    // #[test]
    // fn crear_paquete_default() {
    //     let bytes_packet = [
    //         0x00, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x04, 0xFF, 0x00, 0x0A, 0x00, 0x06, 0x50,
    //         0x52, 0x55, 0x45, 0x42, 0x41,
    //     ];
    //     let prueba = PacketManager::get(&bytes_packet);
    //     assert_eq!(prueba.get_type(), "default".to_owned());
    // }
}