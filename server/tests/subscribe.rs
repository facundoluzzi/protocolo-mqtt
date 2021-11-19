use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::{thread, time};

use server::config_parser::ServerConfigs;
use server::logs::logger::Logger;
use server::server::main::run_server;
use server::topics::topic_manager::TopicManager;

use std::net::TcpListener;

fn setup() {
    match TcpListener::bind("0.0.0.0:1883") {
        Ok(listener) => {
            thread::spawn(move || {
                let mut logger =
                    Logger::new("subscribe-tests.txt".to_string()).expect("Logger could not be created");
                let publish_subscriber_sender = TopicManager::new();
                run_server(&listener, logger, publish_subscriber_sender);
            });
            thread::sleep(time::Duration::from_millis(100));
        },
        Err(_) => {}
    }
}


#[test]
fn connect_should_be_success() {
    setup();
    let mut stream = TcpStream::connect("localhost:1883").unwrap();
    let connect_bytes = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0x00, // flags 
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x00, 0x00, // client identifier
    ];
    stream.write(&connect_bytes).unwrap();
    let mut data = vec![0; 100];
    match stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
        }
        _ => {
            panic!();
        }
    }
}