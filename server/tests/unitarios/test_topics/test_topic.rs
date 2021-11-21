#[cfg(test)]
mod tests {
    use std::sync::mpsc::{self, Receiver, RecvError, Sender};

    use server::topics::topic::Topic;


    #[test]
    fn create_topic_and_send_message_to_subscribers() {
        let mut topic = Topic::new("ALTEGO".to_owned());
        assert!(topic.equals("ALTEGO".to_owned()));
        let (sender_one, receiver_one): (Sender<String>, Receiver<String>) = mpsc::channel();
        let (sender_two, receiver_two): (Sender<String>, Receiver<String>) = mpsc::channel();
        topic.add(sender_one, "Facundo".to_owned());
        topic.add(sender_two, "Nacho".to_owned());
        topic.publish_msg("Bienvenidos a Altego".to_owned());
        
        if let Ok(message_subscriber_one) = receiver_one.recv(){
            assert_eq!(message_subscriber_one, "Bienvenidos a Altego".to_owned());
        }else{
            assert_eq!(0, 1);
        }
        if let Ok(message_subscriber_two) = receiver_two.recv(){
            assert_eq!(message_subscriber_two, "Bienvenidos a Altego".to_owned());
        }else{
            assert_eq!(0, 1);

        }
    }

    #[test]
    fn create_topic_add_two_subscribers_remove_one_and_publish_message() {
        let mut topic = Topic::new("ALTEGO".to_owned());
        assert!(topic.equals("ALTEGO".to_owned()));
        let (sender_one, receiver_one): (Sender<String>, Receiver<String>) = mpsc::channel();
        let (sender_two, receiver_two): (Sender<String>, Receiver<String>) = mpsc::channel();
        topic.add(sender_one, "Facundo".to_owned());
        topic.add(sender_two, "Nacho".to_owned());
        topic.remove("Facundo".to_owned());
        topic.publish_msg("Bienvenidos a Altego".to_owned());
        std::thread::spawn(move || {
            for _recv in receiver_one.recv(){
                assert_eq!(0, 1);
            }
            for recv2 in receiver_two.recv(){
                assert_eq!(recv2, "Bienvenidos a Altego".to_owned());
            }
        });
    }
}