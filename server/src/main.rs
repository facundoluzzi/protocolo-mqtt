use server::packet_factory::PacketFactory;

use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_new_client(mut stream: TcpStream) {
    // TODO: revisar el largo
    let mut data = [0_u8; 50];
    while match stream.read(&mut data) {
        Ok(_size) => {
            let packet = PacketFactory::get(&data);
            packet.send_response(&stream);
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
    println!("Server listening on port 1883");
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
