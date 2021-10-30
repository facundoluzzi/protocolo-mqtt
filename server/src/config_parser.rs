use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

pub struct ServerConfigs {
    configurations: HashMap<String, String>,
}

impl Default for ServerConfigs {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerConfigs {
    pub fn new() -> ServerConfigs {
        ServerConfigs {
            configurations: HashMap::new(),
        }
    }

    fn open_configuration_file(path: String) -> File {
        let file = File::open(path);
        match file {
            Ok(file) => file,
            Err(error_file) => panic!("Error opening config file {}", error_file)
        }
    }

    pub fn charge_configurations_from_path_file(&mut self, path: String) {
        let file = ServerConfigs::open_configuration_file(path);
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

    pub fn get_conf_named(&self, conf_name: String) -> String {
        match self.configurations.get(&conf_name) {
            Some(config_value) => config_value.to_string(),
            None => {
                println!("Theres no configuration with name: {}", conf_name);
                "".to_string()
            }
        }
    }

    pub fn get_config_names(&self) -> Vec<String> {
        let keys = self.configurations.keys().map(|x| x.to_string()).collect();
        keys
    }

    pub fn get_config_values(&self) -> Vec<String> {
        let values = self.configurations.values().map(|x| x.to_string()).collect();
        values
    }
}

#[cfg(test)]
mod test_config_parser {
    use super::*;
    use std::fs;
    use std::io::prelude::*;

    fn create_test_config_file(path: String) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        file.write(b"test1 1\n")?;
        file.write(b"#Comentario test\n")?;
        file.write(b"test2 2\n")?;
        file.write(b"#Comentario test2\n\n")?;
        file.write(b"test3 3\n\n")?;
        file.write(b"test4 4")?;
        Ok(())
    }

    #[test]
    fn configurations_initialize_empty() {
        let configs = ServerConfigs::new();
        assert!(configs.configurations.is_empty());
    }

    #[test]
    fn configurations_does_not_have_commented_lines() {
        create_test_config_file("testParser1.conf".to_string()).unwrap();
        let mut configs = ServerConfigs::new();
        configs.charge_configurations_from_path_file("testParser1.conf".to_string());
        let keys = configs.get_config_names();
        for key in keys {
            assert!(!key.starts_with("#"));
        }
        fs::remove_file("testParser1.conf").unwrap_or(());
    }

    #[test]
    fn configurations_does_not_have_blank_lines() {
        create_test_config_file("testParser2.conf".to_string()).unwrap();
        let mut configs = ServerConfigs::new();
        configs.charge_configurations_from_path_file("testParser2.conf".to_string());
        let keys = configs.get_config_names();
        for key in keys {
            assert!(!key.starts_with(" "));
        }
        fs::remove_file("testParser2.conf").unwrap_or(());
    }

    #[test]
    fn configurations_values_are_correct() {
        create_test_config_file("testParser3.conf".to_string()).unwrap();
        let expected_values = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
        ];
        let mut configs = ServerConfigs::new();
        configs.charge_configurations_from_path_file("testParser3.conf".to_string());
        let mut values = configs.get_config_values();
        values.sort();
        assert_eq!(expected_values, values);
        fs::remove_file("testParser3.conf").unwrap_or(());
    }

    #[test]
    fn configurations_keys_are_correct() {
        create_test_config_file("testParser4.conf".to_string()).unwrap();
        let expected_values = vec![
            "test1".to_string(),
            "test2".to_string(),
            "test3".to_string(),
            "test4".to_string(),
        ];
        let mut configs = ServerConfigs::new();
        configs.charge_configurations_from_path_file("testParser4.conf".to_string());
        let mut keys = configs.get_config_names();
        keys.sort();
        assert_eq!(expected_values, keys);
        fs::remove_file("testParser4.conf").unwrap_or(());
    }
}
