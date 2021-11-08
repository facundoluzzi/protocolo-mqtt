use server::config_parser::ServerConfigs;
use server::logs::logger::Logger;
use server::paquetes::packet_factory::PacketFactory;
use server::topics::topic_manager::TopicManager;
use std::sync::Arc;
use std::sync::Mutex;

use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_new_client(mut stream: TcpStream, mut logger: Logger, topics: Arc<Mutex<TopicManager>>) {
    // TODO: revisar el largo
    let mut data = [0_u8; 100];
    // TODO: ver que onda el while
    let packet_factory = PacketFactory::new();
    while match stream.read(&mut data) {
        Ok(size) => {
            if size == 0 {
                false
            } else {
                logger.info(format!("Received from client {:?}", &data[0..size]));
                packet_factory.process_message(&data[0..size], &*topics.lock().unwrap().get_publish_sender());
                true
            }
        }
        Err(_) => {
            logger.error(format!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            ));
            stream.shutdown(Shutdown::Both).unwrap();
            true
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
    let topics = TopicManager::new();
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
