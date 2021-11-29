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
    use super::*;
    use std::io::Read;
    use std::io::Write;
    use std::net::TcpStream;

    #[test]
    fn should_create_subscribe_packet_succesfully_qos_0() {
        setup();

        let stream = TcpStream::connect("localhost:1883");

        if let Ok(mut stream) = stream {
            let mut data = vec![0; 1000];

            let connect_bytes = [
                0x10, // packet type
                0x0E, // remaining length
                0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
                0x04, // protocol name
                0x00, // flags
                0x00, 0x0B, // keep alive
                0x00, 0x02, 0x00, 0x00 // client identifier
            ];
            stream.write(&connect_bytes).unwrap();
            stream.read(&mut data).unwrap();

            let subscribe_bytes = [
                0x80, // packet type
                0x09, // remaining length
                0x00, 0x0A, // variable header, en particular packet identifier
                0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x00 // payload MQTT como mensaje + qos
            ];

            stream.write(&subscribe_bytes).unwrap();

            match stream.read(&mut data) {
                Ok(size) => {
                    assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
                }
                _ => {
                    panic!();
                }
            }
        }
    }

    #[test]
    fn should_create_subscribe_packet_succesfully_qos_1() {
        setup();

        let stream = TcpStream::connect("localhost:1883");

        if let Ok(mut stream) = stream {
            let mut data = vec![0; 1000];

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
            stream.read(&mut data).unwrap();

            let subscribe_bytes = [
                0x80, // packet type
                0x09, // remaining length
                0x00, 0x0A, // variable header, en particular packet identifier
                0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x01 // payload MQTT como mensaje
            ];

            stream.write(&subscribe_bytes).unwrap();

            match stream.read(&mut data) {
                Ok(size) => {
                    assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x01]);
                }
                _ => {
                    panic!();
                }
            }
        }
    }

    #[test]
    fn should_fail_subscribe_packet_qos_2() {
        setup();

        let stream = TcpStream::connect("localhost:1883");

        if let Ok(mut stream) = stream {
            let mut data = vec![0; 1000];

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
            stream.read(&mut data).unwrap();

            let subscribe_bytes = [
                0x80, // packet type
                0x09, // remaining length
                0x00, 0x0A, // variable header, en particular packet identifier
                0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x02, // payload MQTT como mensaje
            ];

            stream.write(&subscribe_bytes).unwrap();

            match stream.read(&mut data) {
                Ok(size) => {
                    assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x80]);
                }
                _ => {
                    panic!();
                }
            }
        }
    }

    #[test]
    fn should_fail_subscribe_packet_with_a_random_value() {
        setup();

        let stream = TcpStream::connect("localhost:1883");

        if let Ok(mut stream) = stream {
            let mut data = vec![0; 1000];

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
            stream.read(&mut data).unwrap();

            let subscribe_bytes = [
                0x80, // packet type
                0x09, // remaining length
                0x00, 0x0A, // variable header, en particular packet identifier
                0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x03, // payload MQTT como mensaje
            ];

            stream.write(&subscribe_bytes).unwrap();

            match stream.read(&mut data) {
                Ok(size) => {
                    assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x80]);
                }
                _ => {
                    panic!();
                }
            }
        }
    }

    #[test]
    fn should_subscribe_some_topics() {
        setup();

        let stream = TcpStream::connect("localhost:1883");

        if let Ok(mut stream) = stream {
            let mut data = vec![0; 1000];

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
            stream.read(&mut data).unwrap();

            let subscribe_bytes = [
                0x80, // packet type
                0x14, // remaining length
                0x00, 0x0A, // variable header, en particular packet identifier
                0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x03, // payload MQTT como mensaje
                0x00, 0x03, 0x4D, 0x15, 0x45, 0x00, // payload MQT como mensaje
                0x00, 0x02, 0x4D, 0x15, 0x01, // payload MQ como mensaje
            ];

            stream.write(&subscribe_bytes).unwrap();

            match stream.read(&mut data) {
                Ok(size) => {
                    assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x80, 0x00, 0x01]);
                }
                _ => {
                    panic!();
                }
            }
        }
    }
}
