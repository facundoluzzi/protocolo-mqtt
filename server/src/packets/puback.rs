use std::sync::mpsc::Sender;

use crate::enums::user_manager::{
    stop_publish_user_manager::StopPublish, user_manager_action::UserManagerAction,
};
pub struct Puback {
    remaining_length: usize,
    packet_identifier: [u8; 2],
}

impl Puback {
    pub fn init(bytes: &[u8]) -> Puback {
        let variable_header = &bytes[2..4];
        let packet_identifier_msb = variable_header[0];
        let packet_identifier_lsb = variable_header[1];
        Puback {
            remaining_length: 2,
            packet_identifier: [packet_identifier_msb, packet_identifier_lsb],
        }
    }
    pub fn get_packet_identifier(&self) -> [u8; 2] {
        self.packet_identifier
    }
    pub fn stop_publish(&self, client_id: String, sender_user_manager: Sender<UserManagerAction>) {
        let action = UserManagerAction::StopPublishUserManager(StopPublish::init(
            client_id,
            self.packet_identifier,
        ));
        sender_user_manager.send(action).unwrap();
    }
}
