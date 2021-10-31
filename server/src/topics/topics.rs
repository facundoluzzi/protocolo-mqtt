pub struct Topics {
    count: usize,
}

impl Topics {
    pub fn new() -> Topics {
        Topics { count: 0 }
    }

    pub fn add_topic(&mut self) -> usize {
        self.count += 1;
        self.count
    }
}
