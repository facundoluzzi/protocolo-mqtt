use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_new_client(mut stream: TcpStream) {
    let mut data = [0_u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            if let Err(msg_error) = stream.write(&data[0..size]) {
                println!("Error in sending response: {}", msg_error);
            }
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1883").unwrap();
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_new_client(stream));
            }
            Err(e) => {
                println!("Error on connection: {}", e);
            }
        }
    }
    drop(listener);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample_server() {
        assert_eq!(1, 1)
    }
}
