// #[cfg(test)]
// mod tests {
//     use server::paquetes::publisher_suscriber::PublisherSuscriber;
//     use std::sync::mpsc;
//     use std::sync::mpsc::Receiver;
//     use std::sync::mpsc::Sender;
//     use std::sync::Arc;
//     use std::sync::Mutex;
//     use std::thread;

//     use server::helper::publisher_subscriber_code::PublisherSubscriberCode;
//     use server::paquetes::subscribe::Subscribe;

//     #[test]
//     fn should_create_subscribe_packet() {
//         let subscribe_bytes = [
//             0x80, // packet type
//             0x08, // remaining length
//             0x00, 0x0A, // variable header, en particular packet identifier
//             0x00, 0x04, 0x54, 0x54, 0x54, 0x54, 0x00, // payload TTTT como mensaje
//         ];

//         let (sender_one, receiver_one): (Sender<PublisherSuscriber>, Receiver<PublisherSuscriber>) =
//             mpsc::channel();
//         let (sender_two, receiver_two): (Sender<String>, Receiver<String>) = mpsc::channel();

//         let messages: Vec<PublisherSuscriber> = Vec::new();
//         let data = Arc::new(Mutex::new(messages));
//         let data_for_thread = data.clone();

//         let t = thread::spawn(move || {
//             for recv in receiver_one.recv() {
//                 let mut data = data_for_thread.lock().unwrap();
//                 data.push(recv);
//             }
//         });

//         let _subscribe = Subscribe::init(&subscribe_bytes).subscribe_topic(
//             &sender_one,
//             sender_two,
//             "clientId".to_string(),
//         );

//         t.join().unwrap();

//         let locked_data = data.lock().unwrap();
//         let publisher_subscriber_sent = locked_data.get(0).unwrap();
//         assert_eq!(
//             publisher_subscriber_sent.get_packet_type(),
//             PublisherSubscriberCode::Subscriber
//         );
//         assert_eq!(
//             publisher_subscriber_sent.get_client_id(),
//             "clientId".to_string()
//         );

//         let topic = publisher_subscriber_sent.get_topic();

//         assert_eq!(topic, "TTTT".to_owned());
//         assert_eq!(publisher_subscriber_sent.get_message(), "None".to_string());

//         publisher_subscriber_sent
//             .get_sender()
//             .unwrap()
//             .send("message".to_string())
//             .unwrap();

//         assert_eq!(receiver_two.recv().unwrap(), "message".to_string());
//     }
// }
