use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpStream;
use std::net::{TcpListener};


fn main() {
    let address = "0.0.0.0:1883".to_owned();
    let listener = TcpListener::bind(address);
    if let Ok(listener_ok) = listener {
        println!("{:?}", listener_ok);
        let connection = listener_ok.accept();
        if let Ok(connection_ok) = connection {
            let client_stream : TcpStream = connection_ok.0;
            let reader = BufReader::new(client_stream);
            let mut lines = reader.lines();
            while let Some(line) = lines.next() {
                println!("Recibido: {:?}", line);
            }
        } else if let Err(connection_err) = connection {
            println!("{:?}", connection_err);
            return;
        }
    } else if let Err(err) = listener {
        println!("{:?}", err);
        return;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample_client() {
        assert_eq!(1, 1)
    }
}


