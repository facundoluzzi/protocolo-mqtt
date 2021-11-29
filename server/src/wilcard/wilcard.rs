pub struct Wilcard {
    vec_words: Vec<String>,

}

impl Wilcard {
    pub fn init(words: Vec<String>, contains_astherisc: bool, contains_greter_than: bool) -> Wilcard {
        Wilcard { 
            vec_words: words, 
        }
    }

    pub fn verify_topic(&self, topic: String) -> bool {
        let vec_topic: Vec<String> = topic.split('/').map(|s| s.to_string()).collect();
        let counter = 0; 
        if vec_topic.len() < self.vec_words.len() {
            return false;
        }
        for i in self.vec_words {
            if i != "*" {
                if vec_topic[counter] != i {
                    return false;
                }
            }
            if i == ">" {
                return true;
            }
        }
        false
    }
}


