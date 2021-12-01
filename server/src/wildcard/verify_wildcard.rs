use super::wildcard::Wildcard;

pub fn get_wilcard(topic: String) -> Option<Wildcard> {
    let vec_words: Vec<String> = topic.split('/').map(|s| s.to_string()).collect();
    let astherisc = "*".to_owned();
    let greater_than = ">".to_owned();
    let contains_astherisc = vec_words.contains(&astherisc);
    let contains_greater_than = *vec_words.last().unwrap() == greater_than;
    if contains_greater_than {
        let mut vec_copy = vec_words.clone();
        vec_copy.retain(|x| *x == ">".to_owned());
        if vec_copy.len() > 1 {
            return None;
        }
    }
    if contains_astherisc == false && contains_greater_than == false {
        None
    } else {
        Some(Wildcard::init(vec_words))
    }
}
