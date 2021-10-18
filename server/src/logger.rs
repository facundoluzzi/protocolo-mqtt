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

    pub fn info(&mut self, message: String) {
        match self.sender.send(message) {
            Ok(success_message) => {
                println!("{:?}", success_message);
            },
            Err(error_message) => {
                println!("{}", error_message);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{BufRead, BufReader};
    use std::{thread, time};

    #[test]
    fn test_sample_server() {
        match Logger::new("./prueba.txt".to_owned()) {
            Ok(mut success_logger) => {
                success_logger.info("message".to_owned());
            }
            Err(_) => {}
        };

        thread::sleep(time::Duration::from_millis(10));

        let mut vec = Vec::new();
        let file = File::open("./prueba.txt").unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            vec.push(line.unwrap());
        }

        assert_eq!(vec, ["message".to_owned()]);
        fs::remove_file("./prueba.txt".to_owned()).unwrap();
    }
}
