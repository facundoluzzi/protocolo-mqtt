mod tests {
    use std::sync::mpsc::{self, Receiver, Sender};

    use server::{
        paquetes::disconnect::Disconnect,
        stream::stream_handler::{StreamAction, StreamType},
        usermanager::{
            user_manager_action::UserManagerAction, user_manager_types::ChannelUserManager,
        },
    };

    #[test]
    pub fn disconnect_succesfully() {
        let (sender_user_manager, receiver_user_manager): (
            Sender<ChannelUserManager>,
            Receiver<ChannelUserManager>,
        ) = mpsc::channel();

        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) =
            mpsc::channel();

        Disconnect::disconnect_user(
            "TestDisconnect".to_owned(),
            sender_user_manager,
            sender_stream,
        );

        if let Ok(message_user_manager) = receiver_user_manager.recv() {
            match message_user_manager.0 {
                UserManagerAction::DisconnectUserManager => assert_eq!(1, 1),
                _ => assert_eq!(0, 1),
            };
            assert_eq!(message_user_manager.1, "TestDisconnect".to_owned());
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
