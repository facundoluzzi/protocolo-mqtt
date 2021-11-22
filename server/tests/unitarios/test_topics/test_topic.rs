#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::sync::Arc;
    use std::sync::mpsc;
    use std::sync::mpsc::Receiver;
    use std::sync::mpsc::Sender;
    use std::thread;

    use server::topics::topic::Topic;

    #[test]
    fn should_add_topic_and_publish_message() {
        let mut topic = Topic::new("ALTEGO".to_owned());
        assert!(topic.equals("ALTEGO".to_owned()));

        let (sender_one, receiver_one): (Sender<String>, Receiver<String>) = mpsc::channel();
        let (sender_two, receiver_two): (Sender<String>, Receiver<String>) = mpsc::channel();

        topic.add(sender_one, "Facundo".to_owned());
        topic.add(sender_two, "Nacho".to_owned());
        
        let messages: Vec<String> = Vec::new();
        let data = Arc::new(Mutex::new(messages));
        let data_for_thread = data.clone();
        let data_for_second_thread = data.clone();

        let thread_one = thread::spawn(move || {
            let received_data = receiver_one.recv();
            let mut data = data_for_thread.lock().unwrap();
            data.push(received_data.unwrap());
        });

        let thread_two = thread::spawn(move || {
            let received_data = receiver_two.recv();
            let mut data = data_for_second_thread.lock().unwrap();
            data.push(received_data.unwrap());
        });

        topic.publish_msg("hola".to_string());
        thread_one.join().unwrap();
        thread_two.join().unwrap();

        let data = data.lock().unwrap();
        assert_eq!(*data.get(0).unwrap(), "hola".to_string());
        assert_eq!(*data.get(1).unwrap(), "hola".to_string());
    }

    #[test]
    fn create_topic_add_two_subscribers_remove_one_and_publish_message() {
        let mut topic = Topic::new("ALTEGO".to_owned());
        assert!(topic.equals("ALTEGO".to_owned()));

        let (sender_one, receiver_one): (Sender<String>, Receiver<String>) = mpsc::channel();
        let (sender_two, receiver_two): (Sender<String>, Receiver<String>) = mpsc::channel();

        let messages: Vec<String> = Vec::new();
        let data = Arc::new(Mutex::new(messages));
        let data_for_thread = data.clone();

        topic.add(sender_one, "Facundo".to_owned());
        topic.add(sender_two, "Nacho".to_owned());

        topic.remove("Facundo".to_owned());
        
        let t = thread::spawn(move || {
            for _recv in receiver_one.recv() {
                panic!("Should be fail");
            }

            let received_data = receiver_two.recv();
            let mut data = data_for_thread.lock().unwrap();
            if let Err(received_data_fail) = received_data {
                println!("err: {}", received_data_fail);
            }
            if let Ok(received_data_success) = received_data {
                data.push(received_data_success);
            }
        });
        
        topic.publish_msg("Bienvenidos a Altego".to_owned());
        t.join().unwrap();
        let data = data.lock().unwrap();
        assert_eq!(*data.get(0).unwrap(), "Bienvenidos a Altego".to_owned());
    }
}
