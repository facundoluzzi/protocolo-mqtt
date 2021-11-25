// #[cfg(test)]
// mod tests {
//     use server::paquetes::publish::Publish;

//     #[test]
//     fn create_publish_packet_succesfully_with_topic() {
//         let bytes = [
//             0x32, // Paquete publish
//             0x0A, // Remaining Length
//             0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, // Topic
//             0x00, 0x10, // Packet Identifier
//         ];
//         let publish = Publish::init(&bytes);
//         let topic = publish.get_topic();
//         assert_eq!(topic, "ALTEGO".to_owned());
//     }    #[test]
//     fn should_publish_correct_msg() {
//         let bytes = [
//             0x32, // Paquete publish
//             0x0A, // Remaining Length
//             0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, // Topic 'ALTEGO'
//             0x00, 0x10, // Packet Identifier
//             0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, // Payload 'ALTEGO'
//         ];
//         let publish = Publish::init(&bytes);

//         let (sender_one, receiver_one): (Sender<String>, Receiver<String>) = mpsc::channel();
//         let messages: Vec<String> = Vec::new();
//         let data = Arc::new(Mutex::new(messages));
//         let data_for_thread = data.clone();

//         let t = thread::spawn(move || {
//             let received_data = receiver_two.recv();
//             let mut data = data_for_thread.lock().unwrap();
//             if let Err(received_data_fail) = received_data {
//                 println!("err: {}", received_data_fail);
//             }
//             if let Ok(received_data_success) = received_data {
//                 data.push(received_data_success);
//             }
//         });

//         topic.publish_msg("Bienvenidos a Altego".to_owned());
//         t.join().unwrap();
//         let locked_data = data.lock().unwrap();
//         assert_eq!(
//             *locked_data.get(0).unwrap(),
//             "Bienvenidos a Altego".to_owned()
//         );
//     }
// }
