use crate::helper::stream::stream_handler::StreamAction;
use std::sync::mpsc::Sender;

pub type StreamType = (StreamAction, Option<Vec<u8>>, Option<Sender<Vec<u8>>>);
