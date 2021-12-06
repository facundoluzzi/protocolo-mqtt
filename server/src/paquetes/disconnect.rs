use crate::stream::stream_handler::StreamAction::CloseConnectionStream;
use crate::usermanager::user_manager_action::UserManagerAction::DisconnectUserManager;
use crate::usermanager::user_manager_types::ChannelUserManager;

use crate::stream::stream_handler::StreamType;

use std::sync::mpsc::Sender;

pub struct Disconnect {}

impl Disconnect {
    pub fn disconnect_user(
        client_id: String,
        user_manager_sender: Sender<ChannelUserManager>,
        sender_stream: Sender<StreamType>,
    ) {
        if let Err(_msg) =
            user_manager_sender.send((DisconnectUserManager, client_id, None, None, None))
        {
            println!("Error");
        }
        if let Err(_msg) = sender_stream.send((CloseConnectionStream, None, None, None)) {
            println!("Error");
        }
    }
}
