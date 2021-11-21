#[cfg(test)]
mod tests {
    use std::sync::mpsc::{self, Receiver, Sender};
    use server::helper::publisher_subscriber_code::PublisherSubscriberCode;
    use server::paquetes::publisher_suscriber::PublisherSuscriber;

    #[test]
    fn create_publisher_subscriber_succesfully() {
        let publisher_code = PublisherSubscriberCode::Publisher;
        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
        let topic = "ALTEGO".to_owned();
        let message = "A B C".to_owned();
        let publisher_subscriber = PublisherSuscriber::new(topic, message, publisher_code, Some(sender), "jaja".to_owned());

        assert_eq!(publisher_subscriber.get_topic(), "ALTEGO".to_owned());
        assert_eq!(publisher_subscriber.get_message(), "A B C".to_owned());
        match publisher_subscriber.get_packet_type(){
            PublisherSubscriberCode::Publisher => assert_eq!(1,1), 
            _ => assert_eq!(0,1)
        }
        let sender = publisher_subscriber.get_sender();
        sender.unwrap().send("Enviando por Sender".to_string()).unwrap();
        assert_eq!(receiver.recv().unwrap(), "Enviando por Sender".to_string());
    }
}
