use crate::packets::packet_manager::PacketManager;
use std::sync::mpsc::Sender;

use crate::enums::user_manager::{
    stop_publish_user_manager::StopPublish,
    user_manager_action::UserManagerAction::{self},
};
pub struct Puback {
    packet_identifier: [u8; 2],
}

impl Puback {
    /// Recibe los bytes del paquete y el packet manager.
    /// Devuelve Ok(()) o un Err de String en caso de que algo falle
    pub fn process_message(bytes: &[u8], packet_manager: &PacketManager) -> Result<(), String> {
        if packet_manager.is_disconnected() {
            Err("Client is not connected".to_string())
        } else {
            let client_id = packet_manager.get_client_id();
            let sender_user_manager = packet_manager.get_sender_user_manager();

            let puback = Puback::init(bytes)?;
            puback.stop_publish(client_id, sender_user_manager)?;
            Ok(())
        }
    }

    fn init(bytes: &[u8]) -> Result<Puback, String> {
        let variable_header = &bytes[2..4];
        let packet_identifier_msb = variable_header[0];
        let packet_identifier_lsb = variable_header[1];
        Ok(Puback {
            packet_identifier: [packet_identifier_msb, packet_identifier_lsb],
        })
    }

    fn stop_publish(
        &self,
        client_id: String,
        sender_user_manager: Sender<UserManagerAction>,
    ) -> Result<(), String> {
        let action = UserManagerAction::StopPublishUserManager(StopPublish::init(
            client_id,
            self.packet_identifier,
        ));
        if let Err(err) = sender_user_manager.send(action) {
            Err(err.to_string())
        } else {
            Ok(())
        }
    }
}
