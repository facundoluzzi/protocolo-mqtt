use std::io::Write;
use std::net::TcpListener;
use std::thread;
use std::io::Read;

fn main() {
    // conectarse al broker
    // subscribirse al topico de fake generation.
    // guardar los datos en memoria
    // y esperar por peticiones get para devolver todos los datos generados

    // channel

    // let address = format!("{}:{}", self.list_of_inputs[0], self.list_of_inputs[1]);
    // match TcpStream::connect(address) {
    //     Ok(stream) => {
    //         if let Ok(sender_stream) = Stream::init(stream) {
    //             self.send_connect(sender_stream.clone())?;
    //             Ok(sender_stream)
    //         } else {
    //             Err("Error clonando inicializando el stream".to_string())
    //         }
    //     }
    //     Err(err) => {
    //         println!("Failed to connect: {}", err);
    //         let response = "La conexion no se ha podido establecer".to_string();
    //         Err(response)
    //     }
    // }

    // thread::spawn(move || {

    // });

    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let mut data = [0_u8; 400];
                    match stream.read(&mut data) {
                        Ok(size) => {
                            let s = match std::str::from_utf8(&data[0..size]) {
                                Ok(v) => v,
                                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                            };
                            println!("{}", s);

                            let response = "HTTP/1.1 200 OK \n
                            Content-Length: 3 \n
                            Content-Type: text/html; charset=UTF-8 \n
                            asd";

                            println!("{:?}", response);
                            stream.write_all(response.as_bytes()).unwrap();
                        }
                        Err(_err) => {
                        }
                    }
                });
            },
            Err(err) => {

            }
        }
    }
}