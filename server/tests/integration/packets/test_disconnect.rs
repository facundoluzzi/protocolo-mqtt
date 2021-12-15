mod tests {
    use crate::integration::setup::ServerTest;
    use std::{
        io::{Read, Write},
        net::TcpStream,
    };

    #[test]
    pub fn disconnect_and_try_to_read_should_failed() {
        let server = ServerTest::start("0.0.0.0:1893".to_string());
        let mut stream = TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x62, 0x63, // Client Identifier
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
        let disconnect_bytes = [0xE0, 0x00];
        stream.write(&disconnect_bytes).unwrap();

        let subscribe_bytes = [
            0x82, // packet type
            0x08, // remaining length
            0x00, 0x0A, // variable header, en particular packet identifier
            0x00, 0x03, 0x61, 0x2F, 0x62, 0x00, // payload MQTT como mensaje + qos
        ];
        stream.write(&subscribe_bytes).unwrap();
        let mut data2 = vec![0; 100];
        let size = stream.read(&mut data2).unwrap();
        assert_eq!(data2[0..size], []);

        server.shutdown().unwrap();
    }

    #[test]
    pub fn disconnect_and_reconnect() {
        let server = ServerTest::start("0.0.0.0:1894".to_string());
        let mut stream = TcpStream::connect("0.0.0.0:1894".to_string()).unwrap();
        let connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x62, 0x63, // Client Identifier
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

        let disconnect_bytes = [0xE0, 0x00];
        stream.write(&disconnect_bytes).unwrap();

        data = vec![0; 100];
        match stream.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], []);
            }
            _ => {
                panic!();
            }
        }

        stream = TcpStream::connect("0.0.0.0:1894".to_string()).unwrap();
        stream.write(&connect_bytes).unwrap();

        data = vec![0; 100];
        match stream.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes = [
            0x80, // packet type
            0x09, // remaining length
            0x00, 0x0A, // variable header, en particular packet identifier
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x00, // payload MQTT como mensaje + qos
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

        server.shutdown().unwrap();
    }
}
