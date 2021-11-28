use server::usermanager::user_manager::UserManager;
use std::io::Write;
use std::{thread, time};

use server::logs::logger::Logger;
use server::server::main::run_server;
use server::topics::topic_manager::TopicManager;

use std::net::TcpListener;

fn setup() {
    match TcpListener::bind("0.0.0.0:1883") {
        Ok(listener) => {
            thread::spawn(move || {
                let sender_user_manager = UserManager::init();
                let logger = Logger::new("connect-tests.txt".to_string())
                    .expect("Logger could not be created");
                let sender_topic_manager = TopicManager::init();
                run_server(&listener, logger, sender_topic_manager, sender_user_manager);
            });
            thread::sleep(time::Duration::from_millis(100));
        }
        Err(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::net::TcpStream;

    #[test]
    fn should_publish_message_with_qos_0() {
        setup();

        let mut stream = TcpStream::connect("localhost:1883").unwrap();
        let mut data = vec![0; 100];

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

        match stream.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes = [
            0x82, // packet type
            0x08, // remaining length
            0x00, 0x0A, // variable header, en particular packet identifier
            0x00, 0x03, 0x61, 0x2F, 0x62, 0x00, // payload MQTT como mensaje + qos
        ];

        stream.write(&subscribe_bytes).unwrap();

        data = vec![0; 100];
        match stream.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let bytes = [
            0x31, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
            0x0A, // remaining length
            0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
            0x00, 0x0A, // packet identifier
            0x00, 0x03, 0x61, 0x2F, 0x62, // payload
        ];
        stream.write(&bytes).unwrap();
    }

    #[test]
    fn should_publish_message_with_qos_1() {
        setup();
        let stream = TcpStream::connect("localhost:1883");
        if let Ok(mut stream) = stream {
            let bytes = [
                0x32, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
                0x0A, // remaining length
                0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
                0x00, 0x10, // packet identifier
                0x00, 0x03, 0x61, 0x2F, 0x62, // payload
            ];
            stream.write(&bytes).unwrap();

            let mut data = vec![0; 100];

            thread::spawn(move || match stream.read(&mut data) {
                Ok(size) => {
                    assert_eq!(data[0..size], [0x40, 0x01, 0x00, 0x10]);
                }
                _ => {
                    panic!();
                }
            });
        }
    }
}
