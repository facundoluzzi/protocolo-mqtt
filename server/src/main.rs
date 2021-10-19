use server::paquetes::PAQUETES;
use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;

fn get_id_packet(line: String) -> PAQUETES {
    let bytes = line.as_bytes();
    let info_packet = *bytes.get(0).unwrap();
    let id_packet = 0b00001111 & info_packet;
    println!("{:?}", id_packet);
    PAQUETES::CONNECT{}
}

fn main() {
    let address = "0.0.0.0:1883".to_owned();
    let listener = TcpListener::bind(address);
    if let Ok(listener_ok) = listener {
        println!("{:?}", listener_ok);
        let connection = listener_ok.accept();
        if let Ok(connection_ok) = connection {
            let client_stream: TcpStream = connection_ok.0;
            let reader = BufReader::new(client_stream);
            let lines = reader.lines();
            for line in lines {
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
    fn test_sample_server() {
        assert_eq!(1, 1)
    }
}
