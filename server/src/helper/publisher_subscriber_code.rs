#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PublisherSubscriberCode {
    Publisher,
    Subscriber,
    Unsubscriber,
    UnsubscriberAll
}
