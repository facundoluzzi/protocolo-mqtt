mod tests {
    use crate::integration::setup::ServerTest;

    use std::io::Read;
    use std::io::Write;
    use std::net::TcpStream;

    #[test]
    fn should_create_subscribe_packet_succesfully_qos_0_01() {
        let server = ServerTest::start("0.0.0.0:1889".to_string());
        let mut stream = TcpStream::connect("0.0.0.0:1889".to_string()).unwrap();

        let connect_bytes = [
            0x10, // packet type
            0x10, // remaining length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
            0x04, // protocol name
            0x00, // flags
            0x00, 0x0B, // keep alive
            0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // client identifier
        ];

        stream.write(&connect_bytes).unwrap();

        let mut data = vec![0; 1000];
        stream.read(&mut data).unwrap();

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

    #[test]
    fn should_create_subscribe_packet_succesfully_qos_1_02() {
        let server = ServerTest::start("0.0.0.0:1890".to_string());
        let mut stream = TcpStream::connect("0.0.0.0:1890".to_string()).unwrap();

        let mut data = vec![0; 1000];

        let connect_bytes = [
            0x10, // packet type
            0x10, // remaining length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
            0x04, // protocol name
            0x00, // flags
            0x00, 0x0B, // keep alive
            0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // client identifier
        ];
        stream.write(&connect_bytes).unwrap();
        stream.read(&mut data).unwrap();

        let subscribe_bytes = [
            0x80, // packet type
            0x09, // remaining length
            0x00, 0x0A, // variable header, en particular packet identifier
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x01, // payload MQTT como mensaje
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

        server.shutdown().unwrap();
    }

    #[test]
    fn should_fail_subscribe_packet_qos_2_03() {
        let server = ServerTest::start("0.0.0.0:1891".to_string());
        let mut stream = TcpStream::connect("0.0.0.0:1891".to_string()).unwrap();

        let mut data = vec![0; 1000];

        let connect_bytes = [
            0x10, // packet type
            0x10, // remaining length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
            0x04, // protocol name
            0x00, // flags
            0x00, 0x0B, // keep alive
            0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // client identifier
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

        server.shutdown().unwrap();
    }

    #[test]
    fn should_fail_subscribe_packet_with_a_random_value_04() {
        let server = ServerTest::start("0.0.0.0:1892".to_string());
        let mut stream = TcpStream::connect("0.0.0.0:1892".to_string()).unwrap();

        let mut data = vec![0; 1000];

        let connect_bytes = [
            0x10, // packet type
            0x10, // remaining length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
            0x04, // protocol name
            0x00, // flags
            0x00, 0x0B, // keep alive
            0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // client identifier
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

        server.shutdown().unwrap();
    }

    #[test]
    fn should_subscribe_some_topics_05() {
        let server = ServerTest::start("0.0.0.0:1892".to_string());
        let mut stream = TcpStream::connect("0.0.0.0:1892".to_string()).unwrap();

        let mut data = vec![0; 1000];

        let connect_bytes = [
            0x10, // packet type
            0x10, // remaining length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
            0x04, // protocol name
            0x00, // flags
            0x00, 0x0B, // keep alive
            0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // client identifier
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

        server.shutdown().unwrap();
    }
}
