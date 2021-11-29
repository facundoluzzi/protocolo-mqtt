use super::wilcard::Wilcard;

pub fn get_wilcard(topic: String) -> Option<Wilcard> {
    let vec_words: Vec<String> = topic.split('/').map(|s| s.to_string()).collect();
    let astherisc = "*".to_owned();
    let greater_than = ">".to_owned();
    let contains_astherisc = vec_words.contains(&astherisc);
    let contains_greater_than = vec_words.last() == greater_than;
    if contains_greater_than {
        if vec_words.retain(|x| x == ">".to_owned()).len() > 1 {
            return None;
        }
    }
    if contains_astherisc == false && contains_greater_than == false {
        None
    } else {
        Some(Wilcard::init(vec_words, contains_astherisc, contains_greater_than))
    }
}

mod tests {
    use super::*;

    #[test]
    pub fn probando123() {
        let probando = "Futbol/Boca/Hola".to_owned();

        let v: Vec<&str> = probando.split('/').collect();
        println!("{:?}", v);
        assert_eq!(v, ["Futbol", "Boca", "Hola"]);
    }
}


// if d == '>' & (i + 1) == (topic.len() - 1) {
//     // return Some(ToDo!)
// }


// else {
//     if c == '#' && i == (topic.len() - 1) {
//         println!("\n\n\n\n\n\n\n\n\n\nTODOSPUTOS\n\n\n\n\n\n\n\n\n\n\n\n");
//     }
// }