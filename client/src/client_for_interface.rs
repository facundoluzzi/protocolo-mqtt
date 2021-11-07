use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
};

pub struct Client {
    stream: Option<TcpStream>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Client {
        Client { stream: None }
    }

    fn send_connect(self) {
        match self.stream {
            Some(mut stream) => {
                let bytes = [
                    0x10, 0x0E, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x04, 0x00, 0x00, 0x0B, 0x00,
                    0x02, 0x00, 0x00,
                ];
                stream.write_all(&bytes).unwrap();
                {}
            }
            None => panic!("No pude enviar"),
        }
    }

    pub fn connect_to_server(mut self, host: String, port: String) -> Result<String, String> {
        let addres = format!("{}:{}", host, port);
        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let stream_clone = stream.try_clone().expect("Could not clone the stream");
                self.stream = Some(stream_clone);
                self.send_connect();
                thread::spawn(move || {
                    let mut data = vec![0_u8; 100]; // using 6 byte buffer
                    while match stream.read(&mut data) {
                        Ok(size) => {
                            println!("received {:?}", &data[0..size]);
                            true
                        }
                        Err(e) => {
                            println!("Failed to receive data: {}", e);
                            false
                        }
                    } {}
                });
                Ok("La conexion ha sido exitosa".to_string())
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
                Err("La conexion no se ha podido establecer".to_string())
            }
        }
    }
}
