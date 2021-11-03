pub struct Subscriber {}

impl Clone for Subscriber {
    fn clone(&self) -> Self {
        Subscriber {}
    }
}

impl Subscriber {
    pub fn new(_name: String) -> Self {
        Subscriber{}
    }

    pub fn publish_message(self, _message: String) {

    }
}
