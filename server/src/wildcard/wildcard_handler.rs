pub struct Wildcard {
    vec_words: Vec<String>,
}

impl Clone for Wildcard {
    fn clone(&self) -> Self {
        Self {
            vec_words: self.vec_words.clone(),
        }
    }
}

impl Wildcard {
    pub fn init(words: Vec<String>) -> Wildcard {
        Wildcard { vec_words: words }
    }

    pub fn verify_topic(&self, topic: String) -> bool {
        let vec_topic: Vec<String> = topic.split('/').map(|s| s.to_string()).collect();
        let mut counter = 0;
        if vec_topic.len() < self.vec_words.len() {
            return false;
        }
        for i in &self.vec_words {
            let word: Vec<char> = i.chars().collect();
            if i == ">" {
                return true;
            }
            if *word.last().unwrap() == '*' {
                for (n, nword) in word.iter().enumerate().take(word.len() - 1) {
                    let a: u8 = vec_topic[counter].as_bytes()[n];
                    let b: char = a as char;
                    if b != *nword {
                        return false;
                    }
                }
            } else if i != "*" && vec_topic[counter] != *i {
                return false;
            }
            counter += 1;
        }
        // Este es el caso en el que tenemos una Wilcard A/*, y nos llega un paquete A/B/C por ejemplo, A/B cumple, pero debe terminar ahi
        if vec_topic.len() > counter {
            return false;
        }
        true
    }
}
