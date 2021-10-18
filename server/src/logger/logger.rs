use std::fs::File;
use std::io::Write;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

pub struct Logger {
    sender: Sender<String>
}

impl Logger {
    pub fn new(path: String) -> Result<Logger, std::io::Error> {
        let file = File::create(path);
        if let Err(err_file) = file {
            return Err(err_file)
        }

        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

        if let Ok(mut created_file) = file {
            thread::spawn(move || {
                let receive = rx.recv();
                if let Ok(success_receive) = receive {
                    created_file.write_all(success_receive.as_bytes()).unwrap();
                    created_file.write_all(b"\n").unwrap();
                }
            });
        }

        Ok(Logger{ sender: tx })
    }

    pub fn info(&mut self, message: String) {
        self.sender.send(message).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};
    use std::{thread, time};
    use std::fs;

    #[test]
    fn test_sample_server() {
        match Logger::new("./prueba.txt".to_owned()) {
            Ok(mut success_logger) => {
                success_logger.info("message".to_owned());
            },
            Err(_) => {},
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