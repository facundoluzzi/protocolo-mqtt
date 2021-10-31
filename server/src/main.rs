use server::config_parser::ServerConfigs;
use server::logs::logger::Logger;
use server::packet_factory::PacketFactory;
use server::topics::topics::Topics;
use std::sync::Arc;
use std::sync::Mutex;

use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_new_client(mut stream: TcpStream, mut logger: Logger, topics: Arc<Mutex<Topics>>) {
    // TODO: revisar el largo
    let mut data = [0_u8; 100];
    while match stream.read(&mut data) {
        Ok(size) => {
            let mut prueba = topics.lock().unwrap();
            let result = prueba.add_topic();
            logger.info(format!("valor: {}", result));
            // logger.info(format!("Received from client {:?}", &data[0..size]));
            PacketFactory::get(&data[0..size]).send_response(&stream);
            true
        }
        Err(_) => {
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
    let config_path = ServerConfigs::read_config_path_from_cli();
    let server_configs = ServerConfigs::obtain_configurations(config_path);

    let address = format!(
        "0.0.0.0:{}",
        server_configs.get_conf_named("port".to_string())
    );

    let mut logger = Logger::new(server_configs.get_conf_named("log_path".to_string()))
        .expect("Logger could not be created");
    let listener = TcpListener::bind(address).unwrap();
    let topics = Topics::new();
    let mutex = std::sync::Mutex::new(topics);
    let arc = std::sync::Arc::new(mutex);

    logger.info(format!(
        "Server listening on port {}",
        server_configs.get_conf_named("port".to_string())
    ));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                logger.info(format!("New connection: {}", stream.peer_addr().unwrap()));
                let logger_clone = logger.clone();
                let cloned_arc = arc.clone();
                thread::spawn(move || {
                    handle_new_client(stream, logger_clone, cloned_arc);
                });
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
