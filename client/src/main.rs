use core::unicode::conversions::to_lower;
use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

fn main() {
    let address = "0.0.0.0:1883".to_owned();
    let reader = BufReader::new(stdin());
    let socket = TcpStream::connect(address);

    if let Ok(mut socket_ok) = socket {
        for line in reader.lines().flatten() {
            println!("Enviando: {:?}", line);
            let mut toSend :u8 = match line.as_str() {
                "CONNECT" => 0b00010000,
                "CONNAC" => 0b00100000,
                _ => 0b00000000
            };
            match toSend {
                0b00000000 => {println!("NO PACKET");
                    panic!()
                }
                _=> {
                    if let Err(respuesta_uno) = socket_ok.write(toSend) {
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
