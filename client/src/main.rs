use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

fn main() {
    let address = "0.0.0.0:1883".to_owned();
    let reader = BufReader::new(stdin());
    let socket = TcpStream::connect(address);
    println!("{:?}",socket);

    if let Ok(mut socket_ok) = socket {
        for line in reader.lines().flatten() {
            println!("Enviando: {:?}", line);
            let to_send: [u8; 1] = match line.as_str() {
                "CONNECT" => [0b0010000u8],
                "CONNAC" => [0b0100000u8],
                _ => [0b0000000u8]
            };
            match to_send {
                [0b0000000u8] => {println!("NO PACKET");
                }
                _=> {
                    if let Err(respuesta_uno) = socket_ok.write(&to_send) {
                        println!("{}", respuesta_uno);
                        return;
                    }
                }
            } 
        }
    }
    // if let Ok(mut socket_ok) = socket {
    //     for line in reader.lines().flatten() {
    //         println!("Enviando: {:?}", line);
    //         if let Err(respuesta_uno) = socket_ok.write(line.as_bytes()) {
    //             println!("{}", respuesta_uno);
    //             return;
    //         }
    //         if let Err(respuesta_dos) = socket_ok.write("\n".to_owned().as_bytes()) {
    //             println!("{}", respuesta_dos);
    //             return;
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample_client() {
        assert_eq!(1, 1)
    }
}
