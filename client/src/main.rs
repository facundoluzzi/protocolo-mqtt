use std::io::BufRead;
use std::net::TcpStream;
use std::io::stdin;
use std::io::BufReader;
use std::io::Write;

fn main() {
    let address = "0.0.0.0:1883".to_owned();
    let reader = BufReader::new(stdin());
    let mut socket = TcpStream::connect(address);
    if let Ok(mut socket_ok) = socket {
        for line in reader.lines() {
            if let Ok(line_ok) = line {
                println!("Enviando: {:?}", line_ok);
                socket_ok.write(line_ok.as_bytes());
                socket_ok.write("\n".as_bytes());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample_client() {
        assert_eq!(1, 1)
    }
}
