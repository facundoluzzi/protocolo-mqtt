use crate::enums::user_manager::disconnect_user_manager::DisconnectUserManager;
use crate::enums::user_manager::user_manager_action::UserManagerAction;
use crate::stream::stream_handler::StreamAction::CloseConnectionStream;

use crate::stream::stream_handler::StreamType;

use std::sync::mpsc::Sender;

pub struct Disconnect {}

impl Disconnect {
    pub fn disconnect_user(
        client_id: String,
        user_manager_sender: Sender<UserManagerAction>,
        sender_stream: Sender<StreamType>,
    ) {
        let action =
            UserManagerAction::DisconnectUserManager(DisconnectUserManager::init(client_id));
        if let Err(_msg) = user_manager_sender.send(action) {
            println!("Error");
        }
        if let Err(_msg) = sender_stream.send((CloseConnectionStream, None, None, None)) {
            println!("Error");
        }
    }

    // pub fn disconnect_ungracefully(client_id: String,
    //     user_manager_sender: Sender<UserManagerAction>,
    //     sender_stream: Sender<StreamType>,
    //     sender_topic_manager: Sender<TypeMessage>,
    // )

}
