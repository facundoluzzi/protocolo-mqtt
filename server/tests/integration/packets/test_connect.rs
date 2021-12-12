use crate::integration::setup::ServerTest;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

#[test]
fn connect_should_be_success_01() {
    let server = ServerTest::start("0.0.0.0:1883".to_string());
    let mut stream = TcpStream::connect("0.0.0.0:1883".to_string()).unwrap();

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

    let mut data = vec![0; 100];
    let size = stream.read(&mut data).unwrap();

    assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);

    server.shutdown().unwrap();
}

#[test]
fn connect_should_be_success_sending_username_password_02() {
    let server = ServerTest::start("0.0.0.0:1884".to_string());
    let mut stream = TcpStream::connect("0.0.0.0:1884".to_string()).unwrap();

    let connect_bytes = [
        0x10, // packet type
        0x1D, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0xC0, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // client identifier,
        0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, //user
        0x00, 0x03, 0x41, 0x4C, 0x54, // password
    ];

    stream.write(&connect_bytes).unwrap();

    let mut data = vec![0; 100];
    let size = stream.read(&mut data).unwrap();

    assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);

    server.shutdown().unwrap();
}

#[test]
fn connect_should_fail_unacceptable_protocol_version_03() {
    let server = ServerTest::start("0.0.0.0:1885".to_string());
    let mut stream = TcpStream::connect("0.0.0.0:1885".to_string()).unwrap();

    let connect_bytes = [
        0x10, // packet type
        0x10, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x03, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // client identifier,
    ];

    match stream.write(&connect_bytes) {
        Ok(_) => {
            let mut data = vec![0; 100];
            match stream.read(&mut data) {
                Ok(size) => {
                    assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x01]);
                }
                Err(err) => {
                    println!("{}", err);
                    panic!();
                }
            }

            // cuando se lee y devuelve vacío es análogo a un EOL (end of line), es decir que se cerró la conexión. De otro modo quedaría escuchando
            data = vec![0; 100];
            match stream.read(&mut data) {
                Ok(size) => {
                    assert_eq!(data[0..size], []);
                    server.shutdown().unwrap();
                }
                Err(err) => {
                    println!("{}", err);
                    panic!();
                }
            }
        }
        Err(err) => {
            println!("{}", err);
            panic!();
        }
    }
}

#[test]
fn connect_should_fail_username_is_empty_04() {
    let server = ServerTest::start("0.0.0.0:1886".to_string());
    let mut stream = TcpStream::connect("0.0.0.0:1886".to_string()).unwrap();

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

    // cuando se lee y devuelve vacío es análogo a un EOL (end of line), es decir que se cerró la conexión. De otro modo quedaría escuchando
    data = vec![0; 100];
    match stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], []);
        }
        Err(err) => {
            println!("{}", err);
            panic!();
        }
    }

    server.shutdown().unwrap();
}

#[test]
fn connect_should_fail_send_username_without_password_05() {
    let server = ServerTest::start("0.0.0.0:1887".to_string());
    let mut stream = TcpStream::connect("0.0.0.0:1887".to_string()).unwrap();

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

    // cuando se lee y devuelve vacío es análogo a un EOL (end of line), es decir que se cerró la conexión. De otro modo quedaría escuchando
    data = vec![0; 100];
    match stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], []);
        }
        Err(err) => {
            println!("{}", err);
            panic!();
        }
    }

    server.shutdown().unwrap();
}

#[test]
fn connect_should_fail_not_authorized_06() {
    let server = ServerTest::start("0.0.0.0:1888".to_string());
    let mut stream = TcpStream::connect("0.0.0.0:1888".to_string()).unwrap();

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

    // cuando se lee y devuelve vacío es análogo a un EOL (end of line), es decir que se cerró la conexión. De otro modo quedaría escuchando
    data = vec![0; 100];
    match stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], []);
        }
        Err(err) => {
            println!("{}", err);
            panic!();
        }
    }

    server.shutdown().unwrap();
}

#[test]
fn should_fail_when_a_user_connects_twice() {
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
    let mut data = vec![0; 100];
    match stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
        }
        Err(err) => {
            panic!("{}", err.to_string());
        }
    }

    stream.write(&connect_bytes).unwrap();

    data = vec![0; 100];
    match stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], []);
        }
        Err(err) => {
            println!("{}", err);
            panic!();
        }
    }

    server.shutdown().unwrap();
}
