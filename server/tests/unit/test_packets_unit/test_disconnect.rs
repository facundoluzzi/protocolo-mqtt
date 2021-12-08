mod tests {
    use std::sync::mpsc::{self, Receiver, Sender};

    use server::{
        paquetes::disconnect::Disconnect,
        stream::stream_handler::{StreamAction, StreamType},
        usermanager::{
            user_manager_action::UserManagerAction
        },
    };

    #[test]
    pub fn disconnect_succesfully() {
        let (sender_user_manager, receiver_user_manager): (
            Sender<UserManagerAction>,
            Receiver<UserManagerAction>,
        ) = mpsc::channel();

        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) =
            mpsc::channel();

        Disconnect::disconnect_user(
            "TestDisconnect".to_owned(),
            sender_user_manager,
            sender_stream,
        );

        if let Ok(message_user_manager) = receiver_user_manager.recv() {
            match message_user_manager {
                UserManagerAction::DisconnectUserManager(user) => assert_eq!(user.get_client_id(), "TestDisconnect".to_owned()),
                _ => assert_eq!(0, 1),
            };
        } else {
            panic!();
        }

        if let Ok(message_stream_handler) = receiver_stream.recv() {
            match message_stream_handler.0 {
                StreamAction::CloseConnectionStream => assert_eq!(1, 1),
                _ => assert_eq!(0, 1),
            };
        } else {
            panic!();
        }
    }
}
