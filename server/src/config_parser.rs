use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct ConfigParser {
    configurations: HashMap<String, String>,
}

impl Default for ConfigParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigParser {
    pub fn new() -> ConfigParser {
        ConfigParser {
            configurations: HashMap::new(),
        }
    }

    pub fn charge_configurations_from_file(&mut self, file: File) {
        let mut configs: HashMap<String, String> = HashMap::new();
        let lines_in_file = BufReader::new(file).lines();
        let lines_without_comments_and_blanks = lines_in_file
            .map(|line| line.unwrap_or_else(|_| "".to_string()))
            .filter(|line| !line.is_empty() && !line.starts_with('#'));

        let props_and_values = lines_without_comments_and_blanks.map(|line| {
            line.split(' ')
                .map(|word| word.to_string())
                .collect::<Vec<String>>()
        });

        for vec in props_and_values {
            configs.insert(vec[0].to_string(), vec[1].to_string());
        }
        self.configurations = configs;
    }
}

#[cfg(test)]
mod test_config_parser {
    use super::*;

    fn open_config_file() -> Result<File, std::io::Error> {
        let file = File::open("../testParser.conf")?;
        Ok(file)
    }

    #[test]
    fn configurations_initialize_empty() {
        let config_parser = ConfigParser::new();
        assert!(config_parser.configurations.is_empty());
    }

    #[test]
    fn configurations_does_not_have_commented_lines() {
        let file = open_config_file().unwrap();
        let mut config_parser = ConfigParser::new();
        config_parser.charge_configurations_from_file(file);
        for (key, _) in config_parser.configurations {
            assert!(!key.starts_with("#"));
        }
    }

    #[test]
    fn configurations_does_not_have_blank_lines() {
        let file = open_config_file().unwrap();
        let mut config_parser = ConfigParser::new();
        config_parser.charge_configurations_from_file(file);
        for (key, _) in config_parser.configurations {
            assert!(!key.starts_with(" "));
        }
    }

    #[test]
    fn configurations_values_are_correct() {
        let file = open_config_file().unwrap();
        let expected_values = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
        ];
        let mut config_parser = ConfigParser::new();
        config_parser.charge_configurations_from_file(file);
        for (_, value) in config_parser.configurations {
            assert!(expected_values.contains(&value.to_string()));
        }
    }

    #[test]
    fn configurations_keys_are_correct() {
        let file = open_config_file().unwrap();
        let expected_values = vec![
            "test1".to_string(),
            "test2".to_string(),
            "test3".to_string(),
            "test4".to_string(),
        ];
        let mut config_parser = ConfigParser::new();
        config_parser.charge_configurations_from_file(file);
        for (key, _) in config_parser.configurations {
            assert!(expected_values.contains(&key.to_string()));
        }
    }
}
