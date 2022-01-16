use std::sync::mpsc::Sender;
use crate::helper::stream::stream_handler::StreamAction;

pub type StreamType = (StreamAction, Option<Vec<u8>>, Option<Sender<Vec<u8>>>);
