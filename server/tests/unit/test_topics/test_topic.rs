#[cfg(test)]
mod tests {
    use server::topics::topic_actions::TopicAction::{AddTopic, PublishMessage, RemoveTopic};
    use server::usermanager::user_manager_action::UserManagerAction::PublishMessageUserManager;
    use server::usermanager::user_manager_types::ChannelUserManager;
    use std::sync::mpsc;
    use std::sync::mpsc::Receiver;
    use std::sync::mpsc::Sender;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;

    use server::topics::topic::Topic;

    #[test]
    fn should_add_topic_and_publish_message() {
        let mut topic = Topic::init("ALTEGO".to_owned());

        let (sender_one, receiver_one): (Sender<ChannelUserManager>, Receiver<ChannelUserManager>) =
            mpsc::channel();
        let (sender_two, receiver_two): (Sender<ChannelUserManager>, Receiver<ChannelUserManager>) =
            mpsc::channel();

        topic
            .send((AddTopic, "Facundo".to_owned(), Some(sender_one)))
            .unwrap();
        topic
            .send((AddTopic, "Nacho".to_owned(), Some(sender_two)))
            .unwrap();

        topic
            .send((PublishMessage, "hola".to_string(), None))
            .unwrap();

        let (_, client_id, _, _, msg) = receiver_one.recv().unwrap();
        assert_eq!(client_id, "Facundo".to_owned());
        if let Some(msg) = msg {
            assert_eq!(msg, "hola".to_owned());
        } else {
            panic!()
        }

        let (_, client_id, _, _, msg) = receiver_two.recv().unwrap();
        assert_eq!(client_id, "Nacho".to_owned());
        if let Some(msg) = msg {
            assert_eq!(msg, "hola".to_owned());
        } else {
            panic!()
        }
    }
    #[test]
    fn create_topic_add_two_subscribers_remove_one_and_publish_message() {
        let mut topic = Topic::init("ALTEGO".to_owned());

        let (sender_one, receiver_one): (Sender<ChannelUserManager>, Receiver<ChannelUserManager>) =
            mpsc::channel();
        let (sender_two, receiver_two): (Sender<ChannelUserManager>, Receiver<ChannelUserManager>) =
            mpsc::channel();

        topic
            .send((AddTopic, "Facundo".to_owned(), Some(sender_one)))
            .unwrap();
        topic
            .send((AddTopic, "Nacho".to_owned(), Some(sender_two)))
            .unwrap();

        topic
            .send((RemoveTopic, "Facundo".to_owned(), None))
            .unwrap();

        topic
            .send((PublishMessage, "hola".to_string(), None))
            .unwrap();

        for _recv in receiver_one.recv() {
            panic!("Should be fail");
        }

        let (_, client_id, _, _, msg) = receiver_two.recv().unwrap();
        assert_eq!(client_id, "Nacho".to_owned());
        if let Some(msg) = msg {
            assert_eq!(msg, "hola".to_owned());
        } else {
            panic!()
        }
    }
}
