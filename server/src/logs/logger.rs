use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub struct Logger {
    pub sender: Sender<String>,
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        let sender = self.sender.clone();
        Self { sender }
    }
}

impl Logger {
    pub fn new(path: String) -> Result<Logger, std::io::Error> {
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)
        {
            Ok(file) => file,
            Err(err) => panic!("Could not create a logger file: {}", err),
        };

        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

        thread::spawn(move || {
            for receive in rx {
                file.write_all(receive.as_bytes()).unwrap();
                file.write_all(b"\n").unwrap();
            }
        });
        Ok(Logger { sender: tx })
    }

    pub fn error(&mut self, mut error_message: String) {
        error_message = format!("ERROR | {}", error_message);
        if let Err(log_error) = self.sender.send(error_message) {
            println!("There's been an error trying to log: {}", log_error);
        }
    }

    pub fn info(&mut self, mut info_message: String) {
        info_message = format!("INFO | {}", info_message);
        if let Err(log_error) = self.sender.send(info_message) {
            println!("There's been an error trying to log: {}", log_error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::{thread, time};

    fn crear_logger(path: String) -> Result<Logger, std::io::Error> {
        let logger = Logger::new(path.to_owned())?;
        Ok(logger)
    }

    fn assert_that_log_writed_correctly(lines_to_assert: Vec<String>, path: String) {
        let file = File::open(&path).unwrap();
        let mut vec = Vec::new();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            vec.push(line.unwrap());
        }
        assert_eq!(vec, lines_to_assert);
        fs::remove_file(path.to_owned()).unwrap();
    }

    #[test]
    fn test_info_log() {
        let mut logger = crear_logger("./prueba1.txt".to_string()).unwrap();
        logger.info("message".to_owned());
        thread::sleep(time::Duration::from_millis(10));
        assert_that_log_writed_correctly(
            vec!["INFO | message".to_string()],
            "./prueba1.txt".to_string(),
        );
    }

    #[test]
    fn test_error_log() {
        let mut logger = crear_logger("./prueba2.txt".to_string()).unwrap();
        logger.error("message".to_owned());
        thread::sleep(time::Duration::from_millis(10));
        assert_that_log_writed_correctly(
            vec!["ERROR | message".to_string()],
            "./prueba2.txt".to_string(),
        );
    }

    #[test]
    fn test_that_two_loggers_can_send_logs() {
        let mut logger = crear_logger("./prueba.txt".to_string()).unwrap();
        let mut logger2 = logger.clone();
        logger.info("message1".to_owned());
        logger2.info("message2".to_owned());
        thread::sleep(time::Duration::from_millis(10));
        assert_that_log_writed_correctly(
            vec!["INFO | message1".to_string(), "INFO | message2".to_string()],
            "./prueba.txt".to_string(),
        );
    }
}
