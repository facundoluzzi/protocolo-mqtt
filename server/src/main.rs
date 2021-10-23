use server::logger::Logger;
// use server::paquetes;

use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let address = "0.0.0.0:1883".to_owned();
    let listener = TcpListener::bind(address);

    let mut logger = Logger::new("prueba.txt".to_owned()).unwrap();

    if let Ok(listener_ok) = listener {
        println!("{:?}", listener_ok);
        let connection = listener_ok.accept();
        if let Ok(connection_ok) = connection {
            let client_stream: TcpStream = connection_ok.0;
            let reader = BufReader::new(client_stream);
            let lines = reader.lines().flatten();
            for line in lines {
                println!("{:?}", line);
                println!("{}", line);
                // paquetes::PacketBuilder::new(line.clone());
                logger.info(line.clone());
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
    fn test_sample_server() {
        assert_eq!(1, 1)
    }
}
