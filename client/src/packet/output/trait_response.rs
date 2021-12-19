pub trait ResponseTrait {
    fn init(response: String) -> Self
    where
        Self: Sized;
    fn get_response(&self) -> String;
}
