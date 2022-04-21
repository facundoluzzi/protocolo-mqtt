use server::enums::user_manager::user_manager_action::UserManagerAction;
use server::logs::logger::Logger;
use server::server::main::handle_new_client;
use server::stream::stream_handler::Stream;
use server::topic::topic_manager::TopicManager;
use server::usermanager::user_manager::UserManager;
use std::net::TcpListener;
use std::sync::mpsc::Sender;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

pub struct ServerTest {
    server_thread: Option<JoinHandle<Result<(), String>>>,
    sender_user_manager: Sender<UserManagerAction>,
}

impl ServerTest {
    pub fn start(port: String) -> Self {
        let sender_topic_manager = TopicManager::init();
        let sender_user_manager = UserManager::init(sender_topic_manager.clone());

        let cloned_sender_user_manager = sender_user_manager.clone();

        let server_thread: JoinHandle<Result<(), String>> = spawn(move || {
            match TcpListener::bind(port) {
                Ok(listener) => {
                    spawn(move || {
                        let mut logger = Logger::new("setup-tests.txt".to_string())
                            .expect("Logger could not be created");

                        for stream in listener.incoming() {
                            match stream {
                                Ok(stream) => {
                                    logger.info(format!(
                                        "New connection test: {}",
                                        stream.peer_addr().unwrap()
                                    ));

                                    let logger_clone = logger.clone();
                                    let sender_tm_cloned = sender_topic_manager.clone();
                                    let sender_um_cloned = cloned_sender_user_manager.clone();
                                    let sender_stream = Stream::init(stream, logger.clone());

                                    spawn(move || {
                                        handle_new_client(
                                            logger_clone,
                                            sender_stream.unwrap(),
                                            sender_tm_cloned,
                                            sender_um_cloned,
                                        );
                                    });
                                }
                                Err(e) => {
                                    logger.error(format!("Error on connection: {}", e));
                                }
                            }
                        }
                    });

                    sleep(Duration::from_millis(100));
                }
                Err(_) => {}
            }

            Ok(())
        });

        sleep(Duration::from_millis(100));

        Self {
            sender_user_manager: sender_user_manager.clone(),
            server_thread: Some(server_thread),
        }
    }

    pub fn get_sender_user_manager(&self) -> Sender<UserManagerAction> {
        self.sender_user_manager.clone()
    }

    pub fn shutdown(self) -> Result<(), String> {
        match self.server_thread {
            Some(handle) => {
                handle.join().unwrap().unwrap();
            }
            None => {}
        }
        Ok(())
    }
}
