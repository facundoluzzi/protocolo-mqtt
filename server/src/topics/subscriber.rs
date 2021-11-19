#[derive(Debug)]
pub struct Subscriber {
    string: String,
}

impl Clone for Subscriber {
    fn clone(&self) -> Self {
        Subscriber {
            string: "as".to_owned(),
        }
    }
}
impl Default for Subscriber {
    fn default() -> Self {
        Self::new()
    }
}

impl Subscriber {
    pub fn new() -> Subscriber {
        Subscriber {
            string: "as".to_owned(),
        }
    }

    pub fn publish_message(&self, _message: String) {}
}
