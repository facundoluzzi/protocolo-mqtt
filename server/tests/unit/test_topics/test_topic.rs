#[cfg(test)]
mod tests {
    use server::topics::topic_actions::TopicAction::{AddTopic, PublishMessage, RemoveTopic};
    use server::usermanager::user_manager_types::ChannelUserManager;
    use std::sync::mpsc;
    use std::sync::mpsc::Receiver;
    use std::sync::mpsc::Sender;

    use server::topics::topic::Topic;

    #[test]
    fn should_add_topic_and_publish_message() {
        let topic = Topic::init("ALTEGO".to_owned());

        let (sender_one, receiver_one): (Sender<ChannelUserManager>, Receiver<ChannelUserManager>) =
            mpsc::channel();
        let (sender_two, receiver_two): (Sender<ChannelUserManager>, Receiver<ChannelUserManager>) =
            mpsc::channel();

        topic
            .send((AddTopic, Some("Facundo".to_owned()), None, Some(sender_one)))
            .unwrap();
        topic
            .send((AddTopic, Some("Nacho".to_owned()), None, Some(sender_two)))
            .unwrap();

        topic
            .send((
                PublishMessage,
                None,
                Some([0x00, 0x01, 0x02].to_vec()),
                None,
            ))
            .unwrap();

        let (_, client_id, _, _, msg) = receiver_one.recv().unwrap();
        assert_eq!(client_id, "Facundo".to_owned());
        if let Some(msg) = msg {
            assert_eq!(msg, [0x00, 0x01, 0x02].to_vec());
        } else {
            panic!()
        }

        let (_, client_id, _, _, msg) = receiver_two.recv().unwrap();
        assert_eq!(client_id, "Nacho".to_owned());
        if let Some(msg) = msg {
            assert_eq!(msg, [0x00, 0x01, 0x02].to_vec());
        } else {
            panic!()
        }
    }
    #[test]
    fn create_topic_add_two_subscribers_remove_one_and_publish_message() {
        let topic = Topic::init("ALTEGO".to_owned());

        let (sender_one, receiver_one): (Sender<ChannelUserManager>, Receiver<ChannelUserManager>) =
            mpsc::channel();
        let (sender_two, receiver_two): (Sender<ChannelUserManager>, Receiver<ChannelUserManager>) =
            mpsc::channel();

        topic
            .send((AddTopic, Some("Facundo".to_owned()), None, Some(sender_one)))
            .unwrap();
        topic
            .send((AddTopic, Some("Nacho".to_owned()), None, Some(sender_two)))
            .unwrap();

        topic
            .send((RemoveTopic, Some("Facundo".to_owned()), None, None))
            .unwrap();

        topic
            .send((
                PublishMessage,
                None,
                Some([0x00, 0x01, 0x02].to_vec()),
                None,
            ))
            .unwrap();

        for _recv in receiver_one.recv() {
            panic!("Should be fail");
        }

        let (_, client_id, _, _, msg) = receiver_two.recv().unwrap();
        assert_eq!(client_id, "Nacho".to_owned());
        if let Some(msg) = msg {
            assert_eq!(msg, [0x00, 0x01, 0x02].to_vec());
        } else {
            panic!()
        }
    }
}
