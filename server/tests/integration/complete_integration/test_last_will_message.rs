mod tests {
    use std::{
        io::{Read, Write},
        net::TcpStream,
    };
    use core::time;

    use crate::integration::setup::ServerTest;

    #[test]
    fn should_send_will_message_when_disconnect() {
        let server = ServerTest::start("0.0.0.0:1227".to_string());

        let mut stream_to_try_last_will_message =
            TcpStream::connect("0.0.0.0:1227".to_string()).unwrap();
        let mut stream_to_create_and_subscribe_topic =
            TcpStream::connect("0.0.0.0:1227".to_string()).unwrap();

        let connect_bytes_user_two = [
            0x10, // packet type
            0x1D, // remaining length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
            0x04, // protocol name
            0xC0, // flags
            0x00, 0x0B, // keep alive
            0x00, 0x04, 0x62, 0x6F, 0x61, 0x54, // client identifier,
            0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, //user
            0x00, 0x03, 0x41, 0x4C, 0x54, // password
        ];

        let connect_bytes = [
            0x10, // Packet type
            0x2F, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT - Inicio variable header
            0x04, // Protocol Name
            0xE4, // Flags
            0x00, 0x0B, // keep alive - Fin variable header
            0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // CLIENT ID.
            0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // MQTT - Inicio variable header
            0x00, 0x0A, 0x64, 0x69, 0x73, 0x63, 0x6F, 0x6E, 0x6E, 0x65, 0x63,
            0x71, // Message - Disconnect
            0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, // User
            0x00, 0x03, 0x41, 0x4C, 0x54, // Password
        ];

        let mut data = vec![0; 1000];
        stream_to_try_last_will_message
            .write(&connect_bytes)
            .unwrap();
        match stream_to_try_last_will_message.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        data = vec![0; 100];
        stream_to_create_and_subscribe_topic
            .write(&connect_bytes_user_two)
            .unwrap();
        match stream_to_create_and_subscribe_topic.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_topic_mqtt = [
            0x80, // Packet Type
            0x09, // Remaining Length
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // MQTT - Inicio variable header
            0x00,
        ];

        stream_to_create_and_subscribe_topic
            .write(&subscribe_bytes_topic_mqtt)
            .unwrap();
        match stream_to_create_and_subscribe_topic.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes = [
            0x80, // packet type
            0x01, // remaining length
            0x00, // variable header, en particular packet identifier
        ];

        stream_to_try_last_will_message
            .write(&subscribe_bytes)
            .unwrap();


        data = vec![0; 100];
        match stream_to_create_and_subscribe_topic.read(&mut data) {
            Ok(size) => {
                let publish_last_will_message = [
                    0x31, 0x14, 0x00, 0x04, 0x4D, 0x51, 0x54,
                    0x54, // MQTT - Inicio variable header
                    0x00, 0x0A, // Packet ID.
                    0x00, 0x0A, 0x64, 0x69, 0x73, 0x63, 0x6F, 0x6E, 0x6E, 0x65, 0x63,
                    0x71, // Message - Disconnect
                ];
                assert_eq!(data[0..size], publish_last_will_message);
            }
            _ => {
                panic!();
            }
        }

        server.shutdown().unwrap();
    }
}
