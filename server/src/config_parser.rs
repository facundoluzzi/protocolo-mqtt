use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};


pub struct ConfigParser {
    configurations: HashMap<String, String>
}

impl ConfigParser {

    pub fn new() -> ConfigParser {
        ConfigParser {
            configurations: HashMap::new()
        }
    }

    pub fn charge_configurations_from_file(&mut self ,file: File) {
        let mut configs: HashMap<String, String> = HashMap::new();
        let lines_in_file = BufReader::new(file).lines();
        let lines_without_comments_and_blanks = lines_in_file.map(|line| line.unwrap_or("".to_string()))
            .filter(|line| !line.is_empty() && !line.starts_with("#"));
    
        let props_and_values = lines_without_comments_and_blanks.map(|line| {
            line.split(" ").map(|word| word.to_string()).collect::<Vec<String>>()
        });
    
        for vec in props_and_values {
            configs.insert(vec[0].to_string(), vec[1].to_string());
        }
        self.configurations = configs;
    }
}
