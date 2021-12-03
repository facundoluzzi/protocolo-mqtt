use crate::keepalive::trait_keep_alive::TraitKeepAlive;

use std::sync::mpsc::Sender;

pub struct KeepAliveNull {}

impl TraitKeepAlive for KeepAliveNull {
    fn init(_time: u8, _sender: Sender<(String, String)>) -> Box<dyn TraitKeepAlive> {
        Box::new(KeepAliveNull {})
    }

    fn start_keep_alive(&self, _client_id: String, _error_msg: String) {}
}
