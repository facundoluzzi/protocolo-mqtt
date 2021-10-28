use server::logs::logger::Logger;
use server::packet_factory::PacketFactory;

use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_new_client(mut stream: TcpStream) {
    // TODO: revisar el largo
    let mut data = [0_u8; 100];
    while match stream.read(&mut data) {
        Ok(size) => {
            let packet = PacketFactory::get(&data[0..size]);
            packet.send_response(&stream);
            true
        }
        Err(_) => {
            // ESTO FALLA POR QUE NO EXISTE LOGGER
            logger.error(format!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            ));
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let mut logger = Logger::new("test.log".to_string()).expect("Logger could not be created");
    let listener = TcpListener::bind("0.0.0.0:1883").unwrap();
    logger.info("Server listening on port 1883".to_string());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                logger.info(format!("New connection: {}", stream.peer_addr().unwrap()));
                thread::spawn(move || handle_new_client(stream));
            }
            Err(e) => {
                logger.error(format!("Error on connection: {}", e));
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
