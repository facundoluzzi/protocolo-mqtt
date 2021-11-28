use server::config_parser::ServerConfigs;
use server::logs::logger::Logger;
use server::server::main::run_server;
use server::topics::topic_manager::TopicManager;
use server::usermanager::user_manager::UserManager;

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

    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init();

    run_server(&listener, logger, sender_topic_manager, sender_user_manager);
    drop(listener);
}
