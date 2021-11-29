use super::trait_wilcard::Wilcard;

pub struct LastAstherisc {
    pub wilcard: String,
}

impl Wilcard for LastAstherisc {
    fn init (name_wilcard: String) -> Box<dyn Wilcard> {
        Box::new(LastAstherisc {
            wilcard: name_wilcard,
        })
    }
    fn verify_topic(&self, topic_name: String) -> bool {
        let mut contains = false;
        let mut title = "".to_owned();
        for i in 0..topic_name.len() {
            let b = topic_name.as_bytes()[i] as char;
            title.push(b);
            if title == self.wilcard {
                contains = true;
            }
        }
        contains
    }
}