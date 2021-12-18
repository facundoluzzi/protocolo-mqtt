// use std::io::Read;
// use std::io::Write;
// use std::net::TcpStream;

// fn test_01() {
//     let mut stream = TcpStream::connect("localhost:1883").unwrap();
//     let connect_bytes = [
//         0x10, 0x0E, 0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x04, 0x00, 0x00, 0x0B, 0x00, 0x02, 0x00,
//         0x00,
//     ];
//     stream.write(&connect_bytes).unwrap();
//     let mut data = vec![0; 100];
//     match stream.read(&mut data) {
//         Ok(size) => {
//             assert_eq!(data[0..size], [0x20, 0x02, 0x00, 0x00]);
//         }
//         _ => {
//             panic!();
//         }
//     }
// }

// #[test]
// fn integration_tests() {
//     test_01();
// }
