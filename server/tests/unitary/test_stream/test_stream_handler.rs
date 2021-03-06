use server::logs::logger::Logger;
use server::stream::stream_handler::Stream;
use server::stream::stream_handler::StreamAction::CloseConnectionStream;
use server::stream::stream_handler::StreamAction::ReadStream;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::{thread, time};

#[test]
fn should_write_and_read() {
    fn mocked_server(sender: Sender<Vec<u8>>) {
        let logger =
            Logger::new("setup-tests.txt".to_string()).expect("Logger could not be created");
        match TcpListener::bind("0.0.0.0:1895") {
            Ok(listener) => {
                thread::spawn(move || {
                    for stream in listener.incoming() {
                        let sender_stream = Stream::init(stream.unwrap(), logger.clone()).unwrap();
                        sender_stream
                            .send((ReadStream, None, Some(sender.clone()), None))
                            .unwrap();
                    }
                });
            }
            Err(_) => {}
        }
        thread::sleep(time::Duration::from_millis(10));
    }

    let (sender, receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();

    mocked_server(sender);

    let mut stream = TcpStream::connect("localhost:1895").unwrap();

    let bytes: &[u8] = &[
        0x32, 0x0D, 0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, 0x00, 0x10, 0x00, 0x01, 0x41,
    ];

    stream.write(bytes).unwrap();

    match receiver.recv() {
        Ok(msg) => {
            assert_eq!(bytes, msg);
        }
        Err(err) => {
            println!("{}", err);
            panic!();
        }
    }
}

#[test]
fn should_fail_after_close_connection() {
    fn mocked_server(sender: Sender<Vec<u8>>) {
        let logger =
            Logger::new("setup-tests.txt".to_string()).expect("Logger could not be created");
        match TcpListener::bind("0.0.0.0:1896") {
            Ok(listener) => {
                thread::spawn(move || {
                    for stream in listener.incoming() {
                        let sender_stream = Stream::init(stream.unwrap(), logger.clone()).unwrap();

                        sender_stream
                            .send((CloseConnectionStream, None, None, None))
                            .unwrap();

                        sender_stream
                            .send((ReadStream, None, Some(sender.clone()), None))
                            .unwrap();
                    }
                });
                thread::sleep(time::Duration::from_millis(10));
            }
            Err(_) => {}
        }
    }

    let (sender, _receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();

    mocked_server(sender);

    let mut stream = TcpStream::connect("localhost:1896").unwrap();
    thread::sleep(time::Duration::from_millis(10));

    let bytes: &[u8] = &[
        0x32, 0x0D, 0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, 0x00, 0x10, 0x00, 0x01, 0x41,
    ];

    stream.write(bytes).unwrap();
    let mut data = vec![0; 100];
    let size = stream.read(&mut data).unwrap();
    assert_eq!(data[0..size], []);
}
