use super::{wildcard_handler::Wildcard, wildcard_result::WildcardResult};
use crate::wildcard::wildcard_result::WildcardResult::{
    HasNoWildcard, HasWildcard, InvalidWildcard,
};

pub fn get_wilcard(topic: String) -> WildcardResult {
    let vec_words: Vec<String> = topic.split('/').map(|s| s.to_string()).collect();
    let astherisc = "*".to_owned();
    let greater_than = ">".to_owned();
    let mut contains_astherisc = vec_words.contains(&astherisc);
    if !contains_astherisc {
        for word in vec_words.clone() {
            if word.ends_with('*') {
                contains_astherisc = true;
            }
        }
    }
    let contains_greater_than = *vec_words.last().unwrap() == greater_than;
    let mut vec_copy = vec_words.clone();
    vec_copy.pop();
    vec_copy.retain(|x| x == &">".to_owned());
    if vec_copy.len() != 0 {

        return InvalidWildcard;
    }

    if !contains_astherisc && !contains_greater_than {
        HasNoWildcard
    } else {
        HasWildcard(Wildcard::init(vec_words))
    }
}
