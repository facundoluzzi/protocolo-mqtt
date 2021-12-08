#[cfg(test)]
mod tests {
    use server::packets::publish::Publish;
    use server::types::topic_types::TypeTopicManager;
    use server::types::topic_types::TypeTopicManager::Publisher;
    use std::sync::mpsc;
    use std::sync::mpsc::Receiver;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;

    #[test]
    fn should_publish_packet_successfully_with_topic() {
        let bytes = [
            0x32, // Paquete publish
            0x0A, // Remaining Length
            0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, // Topic
            0x00, 0x10, // Packet Identifier
            0x00, 0x01, 0x41,
        ];
        let publish = Publish::init(&bytes);
        let topic = publish.get_topic();
        assert_eq!(topic, "ALTEGO".to_owned());
    }

    #[test]
    fn should_publish_correct_msg() {
        let bytes = [
            0x32, // Paquete publish
            0x09, // Remaining Length
            0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, // Topic 'ALTEGO'
            0x00, 0x10, // Packet Identifier
            0x00, 0x05, 0x41, 0x4C, 0x54, 0x45, 0x47, // Payload 'ALTEG'
        ];

        let (sender_one, receiver_one): (
            std::sync::mpsc::Sender<TypeTopicManager>,
            Receiver<TypeTopicManager>,
        ) = mpsc::channel();

        let messages: Vec<TypeTopicManager> = Vec::new();
        let data = Arc::new(Mutex::new(messages));
        let data_for_thread = data.clone();

        let t = thread::spawn(move || {
            let received_data = receiver_one.recv();
            let mut data = data_for_thread.lock().unwrap();

            if let Ok(received_data_success) = received_data {
                data.push(received_data_success);
            }
        });

        Publish::init(&bytes).send_message(&sender_one, "client_id".to_string());

        t.join().unwrap();
        let locked_data = data.lock().unwrap();
        let type_topic_manager = locked_data.get(0).unwrap();

        match type_topic_manager {
            Publisher(publisher) => {
                assert_eq!(publisher.get_client_id(), "client_id".to_string());
                let topic = publisher.get_topic();
                assert_eq!(topic, "ALTEGO".to_owned());
                assert_eq!(publisher.get_publish_packet(), bytes.to_vec());
            }
            _ => {
                panic!("unexpected error");
            }
        }
    }
}
