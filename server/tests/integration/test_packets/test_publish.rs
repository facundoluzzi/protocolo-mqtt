use server::helper::user_manager::UserManager;
use server::paquetes::publish::Publish;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::{thread, time};

use server::logs::logger::Logger;
use server::server::main::run_server;
use server::topics::topic_manager::TopicManager;

use std::net::TcpListener;

fn setup() {
    match TcpListener::bind("0.0.0.0:1883") {
        Ok(listener) => {
            thread::spawn(move || {
                let user_manager = UserManager::new();
                let logger = Logger::new("connect-tests.txt".to_string())
                    .expect("Logger could not be created");
                let publish_subscriber_sender = TopicManager::new();
                run_server(&listener, logger, publish_subscriber_sender, user_manager);
            });
            thread::sleep(time::Duration::from_millis(100));
        }
        Err(_) => {}
    }
}

#[test]
fn publish_should_be_success() {
    setup();
    let stream = TcpStream::connect("localhost:1883");
    if let Ok(mut stream) = stream{
        let bytes = [
            0x32, 0x07, 0x00, 0x03, 0x61, 0x2F, 0x62, 0x00, 0x10,
        ];
        stream.write(&bytes).unwrap();
        let mut data = vec![0; 100];
        match stream.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x40, 0x02, 0x00, 0x10]);
            }
            _ => {
                panic!();
            }
        }
    }
    
}
