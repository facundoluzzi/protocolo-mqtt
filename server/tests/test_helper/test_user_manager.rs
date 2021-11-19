use std::{thread, time};

use server::helper::user_manager::UserManager;
use server::logs::logger::Logger;
use server::server::main::run_server;
use server::topics::topic_manager::TopicManager;
use std::net::TcpListener;

fn setup() {
    match TcpListener::bind("0.0.0.0:1883") {
        Ok(listener) => {
            thread::spawn(move || {
                let logger =
                    Logger::new("subscribe-tests.txt".to_string()).expect("Logger could not be created");
                let publish_subscriber_sender = TopicManager::new();
                let user_manager = UserManager::new();
                run_server(&listener, logger, publish_subscriber_sender, user_manager);
            });
            thread::sleep(time::Duration::from_millis(100));
        },
        Err(_) => {}
    }
}
#[cfg(test)]
mod tests {
    use std::net::TcpStream;
    use super::*;

    #[test]
    fn add_and_find_a_subscriber() {
        let mut user_manager = UserManager::new();
        let stream = TcpStream::connect("localhost:1883");
        user_manager.add("Pablito".to_owned(), stream.unwrap());
        if let Some(publisher) = user_manager.find_user("Pablito".to_owned()){
            assert_eq!("Pablito".to_owned(), publisher.get_client_id());
        };
        
    }

    #[test]
    fn add_and_find_then_delete_a_subscriber() {
        setup();
        let mut user_manager = UserManager::new();
        
        let stream = TcpStream::connect("localhost:1883");
        
        user_manager.add("Pablito".to_owned(), stream.unwrap().try_clone().unwrap());
        if let Some(publisher) = user_manager.find_user("Pablito".to_owned()){
            assert_eq!("Pablito".to_owned(), publisher.get_client_id());
        };
        user_manager.delete_user("Pablito".to_string());
        let user = user_manager.find_user("Pablito".to_owned());
        assert!(user.is_none());
    }

    #[test]
    fn find_a_non_existent_subscriber_and_get_none() {
        let user_manager = UserManager::new();
        let user = user_manager.find_user("Pablito".to_owned());
        assert!(user.is_none());
    }
}