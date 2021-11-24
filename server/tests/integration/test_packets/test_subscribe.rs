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
                let logger = Logger::new("subscribe-tests.txt".to_string())
                    .expect("Logger could not be created");
                let publish_subscriber_sender = TopicManager::init();
                let user_manager = UserManager::new();
                run_server(&listener, logger, publish_subscriber_sender, user_manager);
            });
            thread::sleep(time::Duration::from_millis(100));
        }
        Err(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use server::paquetes::subscribe::Subscribe;

    use super::*;
    use std::sync::mpsc::{self, Receiver, Sender};

    #[test]
    fn create_subscribe_packet_succesfully() {
        setup();
        let bytes = [0x30, 0x08, 0x00, 0x0A, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45];
        let sender_publisher_subscriber = TopicManager::init();
        let (sender_subscriber, _receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
        let subscribe = Subscribe::init(&bytes).subscribe_topic(
            &sender_publisher_subscriber,
            sender_subscriber,
            "jaja".to_owned(),
        );
        thread::sleep(time::Duration::from_millis(100));
        assert_eq!(subscribe.get_type(), "subscribe".to_owned());
    }
}
