use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use crate::helper::stream::stream_handler::StreamAction;
use crate::packet::sender_type::ClientSender;

pub type SenderForReading = Sender<(Sender<StreamType>, gtk::glib::Sender<ClientSender>)>;
pub type ReceiverForReading = Receiver<(Sender<StreamType>, gtk::glib::Sender<ClientSender>)>;
pub type SenderForServer = Sender<(StreamAction, Option<Vec<u8>>, Option<Sender<Vec<u8>>>)>;
pub type PublishPacket = Result<(String, String, u8, Option<Vec<u8>>), String>;

pub type StreamType = (StreamAction, Option<Vec<u8>>, Option<Sender<Vec<u8>>>);
