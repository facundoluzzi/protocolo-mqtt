use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn open_configuration_file(path: String) -> File {
    let file = File::open(path);
    match file {
        Ok(file) => file,
        Err(error_file) => panic!("Error opening config file {}", error_file),
    }
}

pub fn get_lines_as_key_values(path: String) -> HashMap<String, String> {
    let mut configs: HashMap<String, String> = HashMap::new();
    let file = open_configuration_file(path);
    let lines_in_file = BufReader::new(file).lines();
    let lines_without_comments_and_blanks = lines_in_file
        .map(|line| line.unwrap_or_else(|_| "".to_string()))
        .filter(|line| !line.is_empty() && !line.starts_with('#'));

    let parsed_lines = lines_without_comments_and_blanks.map(|line| {
        line.split(' ')
            .map(|word| word.to_string())
            .collect::<Vec<String>>()
    });

    for vec in parsed_lines {
        configs.insert(vec[0].to_string(), vec[1].to_string());
    }

    configs
}
