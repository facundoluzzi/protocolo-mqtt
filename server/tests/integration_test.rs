use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::{thread, time};

use server::config_parser::ServerConfigs;
use server::logs::logger::Logger;
use server::server_run::run_server;
use server::topics::topic_manager::TopicManager;

use std::net::TcpListener;

mod common;

fn setup() {
    thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:1883").unwrap();
        let mut logger =
            Logger::new("prueba.txt".to_string()).expect("Logger could not be created");
        let publish_subscriber_sender = TopicManager::new();
        run_server(&listener, logger, publish_subscriber_sender);
    });
    thread::sleep(time::Duration::from_millis(100));
}

#[test]
fn integration_tests() {
    setup();
    test_connect_success();
    test_connect_fail_unacceptable_protocol_version();
}

fn test_connect_success() {
    let mut stream = TcpStream::connect("localhost:1883").unwrap();
    let connect_bytes = [
        0x10, 0x0E, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x04, 0x00, 0x00, 0x0B, 0x00, 0x02, 0x00,
        0x00,
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

fn test_connect_fail_unacceptable_protocol_version() {
    let mut stream = TcpStream::connect("localhost:1883").unwrap();
    let connect_bytes = [
        0x10, 0x0E, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x01, 0x00, 0x00, 0x0B, 0x00, 0x02, 0x00,
        0x00,
    ];
    stream.write(&connect_bytes).unwrap();
    let mut data = vec![0; 100];
    match stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x01]);
        }
        _ => {
            panic!();
        }
    }
}
