use crate::enums::topic_manager::publisher::Publisher;
use crate::enums::topic_manager::subscriber::Subscriber;
use crate::enums::topic_manager::unsubscriber::Unsubscriber;
use crate::enums::topic_manager::unsubscriberall::UnsubscriberAll;

pub enum TypeMessage {
    Publisher(Publisher),
    Subscriber(Subscriber),
    Unsubscriber(Unsubscriber),
    UnsubscriberAll(UnsubscriberAll),
}
