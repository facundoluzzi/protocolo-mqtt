#[cfg(test)]
mod tests {
    use server::enums::topic::topic_actions::TopicAction::{AddTopic, PublishMessage, RemoveTopic};
    use server::enums::user_manager::user_manager_action::UserManagerAction;
    use std::sync::mpsc;
    use std::sync::mpsc::Receiver;
    use std::sync::mpsc::Sender;

    use server::topics::topic::Topic;

    #[test]
    fn should_add_topic_and_publish_message() {
        let topic = Topic::init("ALTEGO".to_owned());

        let (sender_one, receiver_one): (Sender<UserManagerAction>, Receiver<UserManagerAction>) =
            mpsc::channel();
        let (sender_two, receiver_two): (Sender<UserManagerAction>, Receiver<UserManagerAction>) =
            mpsc::channel();

        topic
            .send((
                AddTopic,
                Some("Facundo".to_owned()),
                None,
                Some(sender_one),
                0,
                None,
            ))
            .unwrap();
        topic
            .send((
                AddTopic,
                Some("Nacho".to_owned()),
                None,
                Some(sender_two),
                0,
                None,
            ))
            .unwrap();

        topic
            .send((
                PublishMessage,
                None,
                Some([0x00, 0x01, 0x02].to_vec()),
                None,
                0,
                Some(false),
            ))
            .unwrap();

        match receiver_one.recv().unwrap() {
            UserManagerAction::PublishMessageUserManager(user) => {
                assert_eq!(user.get_client_id(), "Facundo".to_owned());
                assert_eq!(user.get_message(), [0x00, 0x01, 0x02].to_vec());
            }
            _ => assert_eq!(0, 1),
        }
        match receiver_two.recv().unwrap() {
            UserManagerAction::PublishMessageUserManager(user) => {
                assert_eq!(user.get_client_id(), "Nacho".to_owned());
                assert_eq!(user.get_message(), [0x00, 0x01, 0x02].to_vec());
            }
            _ => assert_eq!(0, 1),
        }
    }
    #[test]
    fn create_topic_add_two_subscribers_remove_one_and_publish_message() {
        let topic = Topic::init("ALTEGO".to_owned());

        let (sender_one, receiver_one): (Sender<UserManagerAction>, Receiver<UserManagerAction>) =
            mpsc::channel();
        let (sender_two, receiver_two): (Sender<UserManagerAction>, Receiver<UserManagerAction>) =
            mpsc::channel();

        topic
            .send((
                AddTopic,
                Some("Facundo".to_owned()),
                None,
                Some(sender_one),
                0,
                None,
            ))
            .unwrap();
        topic
            .send((
                AddTopic,
                Some("Nacho".to_owned()),
                None,
                Some(sender_two),
                0,
                None,
            ))
            .unwrap();

        topic
            .send((RemoveTopic, Some("Facundo".to_owned()), None, None, 0, None))
            .unwrap();

        topic
            .send((
                PublishMessage,
                None,
                Some([0x00, 0x01, 0x02].to_vec()),
                None,
                0,
                Some(false),
            ))
            .unwrap();

        for _recv in receiver_one.recv() {
            panic!("Should be fail");
        }
        match receiver_two.recv().unwrap() {
            UserManagerAction::PublishMessageUserManager(user) => {
                assert_eq!(user.get_client_id(), "Nacho".to_owned());
                assert_eq!(user.get_message(), [0x00, 0x01, 0x02].to_vec());
            }
            _ => assert_eq!(0, 1),
        }
    }
}
