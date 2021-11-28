use crate::keepalive::trait_keep_alive::TraitKeepAlive;

use std::sync::mpsc::Sender;
use std::thread;
use std::time;

pub struct KeepAlive {
    time: u8,
    sender: Sender<(String, String)>,
}

impl TraitKeepAlive for KeepAlive {
    fn init(time: u8, sender: Sender<(String, String)>) -> Box<dyn TraitKeepAlive> {
        Box::new(KeepAlive { time, sender })
    }

    fn start_keep_alive(&self, _client_id: String, _error_msg: String) {
        let new_time = self.time as u64;
        let cloned_sender = self.sender.clone();
        thread::spawn(move || {
            thread::sleep(time::Duration::from_secs(new_time));
            if let Err(err) = cloned_sender.send(("0".to_string(), "error".to_string())) {
                println!("unexpected err: {}", err);
            }
        });
    }
}
