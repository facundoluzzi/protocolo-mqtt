pub trait Wilcard {
    fn init(name_wilcard: String) -> Box<dyn Wilcard> 
    where
        Self: Sized;
    fn verify_topic(&self, topic_name: String) -> bool;
}