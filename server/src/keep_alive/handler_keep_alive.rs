use crate::stream::stream_handler::StreamAction::SetKeepAliveStream;
use crate::stream::stream_handler::StreamType;
use std::time::Duration;

use std::sync::mpsc::Sender;

/// Configura el keep alive
pub fn init(time: u64, sender: Sender<StreamType>) -> Result<(), String> {
    match sender.send((
        SetKeepAliveStream,
        None,
        None,
        Some(Duration::from_secs(time)),
    )) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
