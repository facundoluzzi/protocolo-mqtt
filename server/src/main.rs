use server::config_parser::ServerConfigs;
use server::helper::user_manager::UserManager;
use server::logs::logger::Logger;
use server::server::main::run_server;
use server::topics::topic_manager::TopicManager;

use std::net::TcpListener;

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

    logger.info(format!(
        "Server listening on port {}",
        server_configs.get_conf_named("port".to_string())
    ));

    let publish_subscriber_sender = TopicManager::new();
    let user_manager = UserManager::new();

    run_server(&listener, logger, publish_subscriber_sender, user_manager);
    drop(listener);
}
