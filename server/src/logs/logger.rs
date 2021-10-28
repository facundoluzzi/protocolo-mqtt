use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub struct Logger {
    pub sender: Sender<String>,
}

impl Logger {
    pub fn new(path: String) -> Result<Logger, std::io::Error> {
        let mut file = File::create(path)?;
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
    use std::io::{BufRead, BufReader};
    use std::{thread, time};

    #[cfg(test)]
    fn crear_logger() -> Result<Logger, std::io::Error> {
        let logger = Logger::new("./prueba.txt".to_owned())?;
        Ok(logger)
    }

    #[test]
    fn test_info_log() {
        let mut logger = crear_logger().unwrap();
        logger.info("message".to_owned());
        thread::sleep(time::Duration::from_millis(10));
        let file = File::open("./prueba.txt").unwrap();
        let mut vec = Vec::new();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            vec.push(line.unwrap());
        }

        assert_eq!(vec, ["INFO | message".to_string()]);
        fs::remove_file("./prueba.txt".to_owned()).unwrap();
    }
}
