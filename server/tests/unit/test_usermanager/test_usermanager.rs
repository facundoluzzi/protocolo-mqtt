use std::sync::mpsc::Sender;

#[cfg(test)]
mod tests {
    use server::stream::stream_handler::StreamType;
    use server::topics::topic_manager::TopicManager;
    use server::usermanager::user_manager::UserManager;
    use server::usermanager::user_manager_action::UserManagerAction::AddUserManager;
    use server::usermanager::user_manager_action::UserManagerAction::DisconnectUserManager;
    use server::usermanager::user_manager_action::UserManagerAction::PublishMessageUserManager;
    use std::sync::mpsc;
    use std::sync::mpsc::Receiver;

    use super::*;

    #[test]
    fn should_add_a_user_and_publish_message() {
        let sender_topic_manager = TopicManager::init();
        let sender = UserManager::init(sender_topic_manager);

        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) =
            mpsc::channel();
        sender
            .send((
                AddUserManager,
                "Nacho".to_owned(),
                Some(sender_stream),
                Some(true),
                None,
            ))
            .unwrap();

        sender
            .send((
                PublishMessageUserManager,
                "Nacho".to_owned(),
                None,
                None,
                Some("mensaje enviado".to_string()),
            ))
            .unwrap();

        let (_, vec, _, _) = receiver_stream.recv().unwrap();

        assert_eq!(
            vec.unwrap(),
            [109, 101, 110, 115, 97, 106, 101, 32, 101, 110, 118, 105, 97, 100, 111].to_vec()
        );
    }

    #[test]
    fn should_add_a_user_and_remove_cant_publish_message() {
        let sender_topic_manager = TopicManager::init();
        let sender = UserManager::init(sender_topic_manager);

        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) =
            mpsc::channel();

        sender
            .send((
                AddUserManager,
                "Nacho".to_owned(),
                Some(sender_stream),
                Some(true),
                None,
            ))
            .unwrap();

        sender
            .send((DisconnectUserManager, "Nacho".to_owned(), None, None, None))
            .unwrap();

        sender
            .send((
                PublishMessageUserManager,
                "Nacho".to_owned(),
                None,
                None,
                Some("mensaje enviado".to_string()),
            ))
            .unwrap();

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
    fn should_add_a_user_and_disconnect_publish_message_send_nothing() {
        let sender_topic_manager = TopicManager::init();
        let sender = UserManager::init(sender_topic_manager);

        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) =
            mpsc::channel();

        sender
            .send((
                AddUserManager,
                "Nacho".to_owned(),
                Some(sender_stream),
                Some(false),
                None,
            ))
            .unwrap();

        sender
            .send((DisconnectUserManager, "Nacho".to_owned(), None, None, None))
            .unwrap();

        sender
            .send((
                PublishMessageUserManager,
                "Nacho".to_owned(),
                None,
                None,
                Some("mensaje enviado".to_string()),
            ))
            .unwrap();

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
        let sender_topic_manager = TopicManager::init();
        let sender = UserManager::init(sender_topic_manager);

        let (sender_stream, _): (Sender<StreamType>, Receiver<StreamType>) = mpsc::channel();

        sender
            .send((
                AddUserManager,
                "Nacho".to_owned(),
                Some(sender_stream.clone()),
                Some(false),
                None,
            ))
            .unwrap();

        sender
            .send((DisconnectUserManager, "Nacho".to_owned(), None, None, None))
            .unwrap();

        sender
            .send((
                PublishMessageUserManager,
                "Nacho".to_owned(),
                None,
                None,
                Some("mensaje enviado".to_string()),
            ))
            .unwrap();

        let (sender_stream_two, receiver_stream_two): (Sender<StreamType>, Receiver<StreamType>) =
            mpsc::channel();

        sender
            .send((
                AddUserManager,
                "Nacho".to_owned(),
                Some(sender_stream_two.clone()),
                Some(false),
                None,
            ))
            .unwrap();

        let (_, vec, _, _) = receiver_stream_two.recv().unwrap();

        assert_eq!(
            vec.unwrap(),
            [109, 101, 110, 115, 97, 106, 101, 32, 101, 110, 118, 105, 97, 100, 111].to_vec()
        );
    }
}
