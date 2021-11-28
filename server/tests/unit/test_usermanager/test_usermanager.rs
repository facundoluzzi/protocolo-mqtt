// use std::sync::mpsc::Sender;
// use server::usermanager::user_manager_types::ChannelUserManager;
// use std::{thread, time};

// use server::usermanager::user_manager::UserManager;
// use server::logs::logger::Logger;
// use server::server::main::run_server;
// use server::topics::topic_manager::TopicManager;
// use std::net::TcpListener;
// use std::sync::mpsc;
// use std::io::Read;

// fn setup(sender_user_manager: Sender<ChannelUserManager>) {
//     match TcpListener::bind("0.0.0.0:1883") {
//         Ok(listener) => {
//             thread::spawn(move || {
//                 let logger =
//                     Logger::new("subscribe-tests.txt".to_string()).expect("Logger could not be created");
//                 let sender_publisher_subscriber = TopicManager::init();
//                 run_server(&listener, logger, sender_publisher_subscriber, sender_user_manager);
//             });
//             thread::sleep(time::Duration::from_millis(100));
//         },
//         Err(_) => {}
//     }
// }

// #[cfg(test)]
// mod tests {
//     use std::sync::mpsc::Receiver;
// use server::usermanager::user_manager_action::UserManagerAction::GetSender;
// use server::usermanager::user_manager_action::UserManagerAction::Add;
// use std::net::TcpStream;
//     use super::*;

//     #[test]
//     fn should_create_a_user_and_publish_a_message() {
//         println!("0");
//         let sender_user_manager = UserManager::init();
//         setup(sender_user_manager.clone());

//         let mut stream = TcpStream::connect("localhost:1883").unwrap();
//         let (sender, receiver): (Sender<Sender<String>>, Receiver<Sender<String>>) = mpsc::channel();

//         sender_user_manager.clone().send((Add, "pablito".to_string(), Some(stream), Some(false), None)).unwrap();
//         sender_user_manager.clone().send((GetSender, "pablito".to_string(), None, None, Some(sender))).unwrap();

//         let sender_for_publish = receiver.recv().unwrap();

//         sender_for_publish.send("MENSAJE PUBLICADO".to_string()).unwrap();

//         let mut data = [0; 100];

//         match stream.read(&mut data) {
//             Ok(readed_bytes) => {
//                 let message = match std::str::from_utf8(&data[0..readed_bytes]) {
//                     Ok(v) => v,
//                     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
//                 };

//                 assert_eq!(message, "MENSAJE PUBLICADO".to_string());
//             },
//             Err(err) => {
//                 panic!(err);
//             }
//         };
//     }

//     // #[test]
//     // fn should_not_find_a_removed_subscriber() {
//     //     setup();
//     //     let mut user_manager = UserManager::init();

//     //     let stream = TcpStream::connect("localhost:1883");
//     //     user_manager.add("Pablito".to_owned(), stream.unwrap().try_clone().unwrap());
//     //     if let Some(publisher) = user_manager.find_user("Pablito".to_owned()){
//     //         assert_eq!("Pablito".to_owned(), publisher.get_client_id());
//     //     };
//     //     user_manager.delete_user("Pablito".to_string());
//     //     let user = user_manager.find_user("Pablito".to_owned());
//     //     assert!(user.is_none());
//     // }

//     // #[test]
//     // fn should_not_find_uncreated_subscriber() {
//     //     setup();
//     //     let user_manager = UserManager::new();
//     //     let user = user_manager.find_user("Pablito".to_owned());
//     //     assert!(user.is_none());
//     // }
// }
// // use std::{thread, time};

// // use server::helper::user_manager::UserManager;
// // use server::logs::logger::Logger;
// // use server::server::main::run_server;
// // use server::topics::topic_manager::TopicManager;
// // use std::net::TcpListener;

// // fn setup() {
// //     match TcpListener::bind("0.0.0.0:1883") {
// //         Ok(listener) => {
// //             thread::spawn(move || {
// //                 let logger = Logger::new("subscribe-tests.txt".to_string())
// //                     .expect("Logger could not be created");
// //                 let publish_subscriber_sender = TopicManager::init();
// //                 let user_manager = UserManager::new();
// //                 run_server(&listener, logger, publish_subscriber_sender, user_manager);
// //             });
// //             thread::sleep(time::Duration::from_millis(100));
// //         }
// //         Err(_) => {}
// //     }
// // }
// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //     use std::net::TcpStream;

// //     #[test]
// //     fn should_find_the_created_subscriber() {
// //         setup();
// //         let mut user_manager = UserManager::new();
// //         let stream = TcpStream::connect("localhost:1883");
// //         user_manager.add("Pablito".to_owned(), stream.unwrap(), true);
// //         if let Some(publisher) = user_manager.find_user("Pablito".to_owned()) {
// //             assert_eq!("Pablito".to_owned(), publisher.get_client_id());
// //         };
// //     }

// //     #[test]
// //     fn should_not_find_a_removed_subscriber() {
// //         setup();
// //         let mut user_manager = UserManager::new();

// //         let stream = TcpStream::connect("localhost:1883");
// //         user_manager.add(
// //             "Pablito".to_owned(),
// //             stream.unwrap().try_clone().unwrap(),
// //             true,
// //         );
// //         if let Some(publisher) = user_manager.find_user("Pablito".to_owned()) {
// //             assert_eq!("Pablito".to_owned(), publisher.get_client_id());
// //         };
// //         user_manager.delete_user("Pablito".to_string());
// //         let user = user_manager.find_user("Pablito".to_owned());
// //         assert!(user.is_none());
// //     }

// //     #[test]
// //     fn should_not_find_uncreated_subscriber() {
// //         setup();
// //         let user_manager = UserManager::new();
// //         let user = user_manager.find_user("Pablito".to_owned());
// //         assert!(user.is_none());
// //     }
// // }
