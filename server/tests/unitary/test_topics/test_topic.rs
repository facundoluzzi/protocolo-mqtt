#[cfg(test)]
mod tests {
    use server::enums::topic::add_topic::AddTopic;
    use server::enums::topic::publish_message::PublishMessage;
    use server::enums::topic::remove_topic::RemoveTopic;
    use server::enums::topic::topic_actions::TopicAction;
    use server::enums::user_manager::user_manager_action::UserManagerAction;
    use std::sync::mpsc;
    use std::sync::mpsc::Receiver;
    use std::sync::mpsc::Sender;

    use server::topic::topics::Topic;

    #[test]
    fn should_add_topic_and_publish_message() {
        let topic = Topic::init("ALTEGO".to_owned());

        let (sender_one, receiver_one): (Sender<UserManagerAction>, Receiver<UserManagerAction>) =
            mpsc::channel();
        let (sender_two, receiver_two): (Sender<UserManagerAction>, Receiver<UserManagerAction>) =
            mpsc::channel();
        let add = TopicAction::Add(AddTopic::init("Facundo".to_owned(), sender_one, 0));
        topic.send(add).unwrap();
        let add = TopicAction::Add(AddTopic::init("Nacho".to_owned(), sender_two, 0));
        topic.send(add).unwrap();
        let publish =
            TopicAction::Publish(PublishMessage::init([0x00, 0x01, 0x02].to_vec(), 0, false));
        topic.send(publish).unwrap();

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
        let add = TopicAction::Add(AddTopic::init("Facundo".to_owned(), sender_one, 0));
        topic.send(add).unwrap();
        let add = TopicAction::Add(AddTopic::init("Nacho".to_owned(), sender_two, 0));
        topic.send(add).unwrap();
        let remove = TopicAction::Remove(RemoveTopic::init("Facundo".to_owned()));
        topic.send(remove).unwrap();

        let publish =
            TopicAction::Publish(PublishMessage::init([0x00, 0x01, 0x02].to_vec(), 0, false));
        topic.send(publish).unwrap();

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
