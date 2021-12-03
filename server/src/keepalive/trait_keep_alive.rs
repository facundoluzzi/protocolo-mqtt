use std::sync::mpsc::Sender;

pub trait TraitKeepAlive {
    fn init(time: u8, sender: Sender<(String, String)>) -> Box<dyn TraitKeepAlive>
    where
        Self: Sized;
    fn start_keep_alive(&self, client_id: String, error_msg: String);
}
