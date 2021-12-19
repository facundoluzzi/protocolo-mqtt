use std::sync::mpsc::Sender;

#[cfg(test)]
mod tests {
    use server::enums::user_manager::add_user_manager::AddUserManager;
    use server::enums::user_manager::disconnect_user_manager::DisconnectUserManager;
    use server::enums::user_manager::publish_message_user_manager::PublishMessageUserManager;
    use server::enums::user_manager::user_manager_action::UserManagerAction;
    use server::stream::stream_handler::StreamType;
    use server::topic::topic_manager::TopicManager;
    use server::usermanager::user_manager::UserManager;
    use std::sync::mpsc;
    use std::sync::mpsc::Receiver;

    use super::*;

    #[test]
    fn should_add_a_user_and_publish_message() {
        let sender_topic_manager = TopicManager::init();
        let sender = UserManager::init(sender_topic_manager);

        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) =
            mpsc::channel();
        let action_to_add = UserManagerAction::AddUserManager(AddUserManager::init_without_will(
            "Nacho".to_owned(),
            sender_stream,
            true,
        ));
        let action_to_publish = UserManagerAction::PublishMessageUserManager(
            PublishMessageUserManager::init("Nacho".to_owned(), [0x00, 0x01, 0x02].to_vec()),
        );
        sender.send(action_to_add).unwrap();
        sender.send(action_to_publish).unwrap();

        let (_, vec, _, _) = receiver_stream.recv().unwrap();

        assert_eq!(vec, Some([0x00, 0x01, 0x02].to_vec()));
    }

    #[test]
    fn should_add_a_user_and_disconnect_publish_message_send_nothing() {
        let sender_topic_manager = TopicManager::init();
        let sender = UserManager::init(sender_topic_manager);

        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) =
            mpsc::channel();
        let action_to_add = UserManagerAction::AddUserManager(AddUserManager::init_without_will(
            "Nacho".to_owned(),
            sender_stream,
            false,
        ));
        let action_to_disconnect = UserManagerAction::DisconnectUserManager(
            DisconnectUserManager::init("Nacho".to_owned(), false),
        );
        let action_to_publish = UserManagerAction::PublishMessageUserManager(
            PublishMessageUserManager::init("Nacho".to_owned(), [0x00, 0x01, 0x02].to_vec()),
        );
        sender.send(action_to_add).unwrap();
        sender.send(action_to_disconnect).unwrap();
        sender.send(action_to_publish).unwrap();

        match receiver_stream.recv() {
            Err(err) => {
                assert_eq!(err.to_string(), "receiving on a closed channel".to_string());
            }
            Ok(_) => {
                panic!();
            }
        }
    }

    #[test]
    fn should_add_a_user_and_disconnect_and_reconnect_publish_message() {
        let bytes = [
            0x30, // tiene la información del packet type 0010, dup flag + qos flag + retain flag
            0x0C, // remaining length
            0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
            0x00, 0x0A, // packet identifier
            0x00, 0x03, 0x61, 0x2F, 0x61, // payload
        ];

        let sender_topic_manager = TopicManager::init();
        let sender = UserManager::init(sender_topic_manager);
        let (sender_stream, _): (Sender<StreamType>, Receiver<StreamType>) = mpsc::channel();
        let action_to_add = UserManagerAction::AddUserManager(AddUserManager::init_without_will(
            "Nacho".to_owned(),
            sender_stream,
            false,
        ));
        let action_to_disconnect = UserManagerAction::DisconnectUserManager(
            DisconnectUserManager::init("Nacho".to_owned(), false),
        );
        let action_to_publish = UserManagerAction::PublishMessageUserManager(
            PublishMessageUserManager::init("Nacho".to_owned(), bytes.to_vec()),
        );
        sender.send(action_to_add).unwrap();

        sender.send(action_to_disconnect).unwrap();

        sender.send(action_to_publish).unwrap();

        let (sender_stream_two, receiver_stream_two): (Sender<StreamType>, Receiver<StreamType>) =
            mpsc::channel();
        let action_to_add_for_reconnect = UserManagerAction::AddUserManager(
            AddUserManager::init_without_will("Nacho".to_owned(), sender_stream_two, false),
        );
        sender.send(action_to_add_for_reconnect).unwrap();

        let (_, vec, _, _) = receiver_stream_two.recv().unwrap();

        assert_eq!(vec, Some(bytes.to_vec()));
    }
}
