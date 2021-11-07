use crate::helper::file_handler::get_lines_as_key_values;
use std::collections::HashMap;
use std::env;

pub struct ServerConfigs {
    configurations: HashMap<String, String>,
}

impl Default for ServerConfigs {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerConfigs {
    fn new() -> ServerConfigs {
        ServerConfigs {
            configurations: HashMap::new(),
        }
    }

    pub fn read_config_path_from_cli() -> String {
        let args: Vec<String> = env::args().collect();
        if args.len() <= 1 {
            panic!("The path to the config file is needed");
        }

        args[1].to_string()
    }

    pub fn obtain_configurations(path: String) -> ServerConfigs {
        let mut server_configs = ServerConfigs::new();
        server_configs.charge_configurations_from_path_file(path);
        server_configs
    }

    fn charge_configurations_from_path_file(&mut self, path: String) {
        self.configurations = get_lines_as_key_values(path);
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
        let values = self
            .configurations
            .values()
            .map(|x| x.to_string())
            .collect();
        values
    }
}

#[cfg(test)]
mod test_config_parser {
    use super::*;
    use std::fs;
    use std::fs::File;
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
        let configs = ServerConfigs::obtain_configurations("testParser1.conf".to_string());
        let keys = configs.get_config_names();
        for key in keys {
            assert!(!key.starts_with("#"));
        }
        fs::remove_file("testParser1.conf").unwrap_or(());
    }

    #[test]
    fn configurations_does_not_have_blank_lines() {
        create_test_config_file("testParser2.conf".to_string()).unwrap();
        let configs = ServerConfigs::obtain_configurations("testParser2.conf".to_string());
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
        let configs = ServerConfigs::obtain_configurations("testParser3.conf".to_string());
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
        let configs = ServerConfigs::obtain_configurations("testParser4.conf".to_string());
        let mut keys = configs.get_config_names();
        keys.sort();
        assert_eq!(expected_values, keys);
        fs::remove_file("testParser4.conf").unwrap_or(());
    }
}
