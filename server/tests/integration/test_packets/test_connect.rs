use server::helper::user_manager::UserManager;
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
fn connect_should_be_success() {
    setup();
    let stream = TcpStream::connect("localhost:1883");
    if let Ok(mut stream) = stream {
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
}

#[test]
fn connect_should_be_success_sending_username_password() {
    setup();
    let mut stream = TcpStream::connect("localhost:1883").unwrap();
    let connect_bytes = [
        0x10, // packet type
        0x1B, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0xC0, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x00, 0x00, // client identifier,
        0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, //user
        0x00, 0x03, 0x41, 0x4C, 0x54, // password
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

#[test]
fn connect_should_fail_unacceptable_protocol_version() {
    setup();
    let mut stream = TcpStream::connect("localhost:1883").unwrap();
    let connect_bytes = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x03, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x00, 0x00, // client identifier
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

#[test]
fn connect_should_fail_username_is_empty() {
    setup();
    let mut stream = TcpStream::connect("localhost:1883").unwrap();
    let connect_bytes = [
        0x10, // packet type
        0x11, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0xB0, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x00, 0x00, // client identifier,
        0x00, 0x00, 0x00, // user
    ];

    stream.write(&connect_bytes).unwrap();
    let mut data = vec![0; 100];
    match stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x04]);
        }
        _ => {
            panic!();
        }
    }
}

#[test]
fn connect_should_fail_send_username_without_password() {
    setup();
    let mut stream = TcpStream::connect("localhost:1883").unwrap();
    let connect_bytes = [
        0x10, // packet type
        0x11, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0xB0, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x00, 0x00, // client identifier,
        0x00, 0x01, 0x45, // user
    ];

    stream.write(&connect_bytes).unwrap();
    let mut data = vec![0; 100];
    match stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x04]);
        }
        _ => {
            panic!();
        }
    }
}

#[test]
fn connect_should_fail_not_authorized() {
    setup();
    let mut stream = TcpStream::connect("localhost:1883").unwrap();
    let connect_bytes = [
        0x10, // packet type
        0x1B, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0xC0, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x00, 0x00, // client identifier,
        0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, //user
        0x00, 0x03, 0x41, 0x4C, 0x53, // password
    ];

    stream.write(&connect_bytes).unwrap();
    let mut data = vec![0; 100];
    match stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x05]);
        }
        _ => {
            panic!();
        }
    }
}
