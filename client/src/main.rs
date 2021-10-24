use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    let reader = BufReader::new(stdin());
    match TcpStream::connect("localhost:1883") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            for line in reader.lines().flatten() {
                if let Err(msg_error) = stream.write(line.as_bytes()) {
                    println!("{}", msg_error);
                    return;
                }
                println!("Sent {}, awaiting reply...", line);

                let mut data = vec![0_u8; line.len()]; // using 6 byte buffer
                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        println!("received {:?}", data);
                        if data == line.as_bytes() {
                            println!("Reply is ok!");
                        } else {
                            let text = from_utf8(&data).unwrap();
                            println!("Unexpected reply: {}", text);
                        }
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample_client() {
        assert_eq!(1, 1)
    }
}
