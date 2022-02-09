use std::io::Write;
use std::net::TcpStream;
use rand::Rng;
use std::thread;
use std::time::Duration;

/// Construye el vector de bytes que van a representar al paquete Publish y los devuelve
fn build_bytes_for_connect() -> Vec<u8> {
    [
        0x10, 
        0x12,
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 
        0x04, 
        0x00, 
        0x00, 0x00, 
        0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, 
    ].to_vec()
}

/// Obtiene la serie de bytes que van a representar al paquete Connect, y manda una accion de escritura hacia el Stream, para que
/// lo mande hacia el broker
fn send_connect(mut stream: TcpStream) -> Result<(), String> {
    let connect_bytes = build_bytes_for_connect();
    if let Err(_err) = stream.write(&connect_bytes) {
        return Err("Error enviando connect".to_string())
    }
    Ok(())
}

pub fn connect_to_server() -> Result<TcpStream, String> {
    let address = "localhost:1883".to_owned();
    match TcpStream::connect(address) {
        Ok(stream) => {
            if let Ok(stream_clone) = stream.try_clone() {
                if let Err(err) = send_connect(stream_clone) {
                    return Err(err);
                } else {
                    return Ok(stream);
                }
            } else {
                return Err("No se pudo establecer la conexion".to_string());
            }
        }
        Err(err) => {
            println!("Failed to connect: {}", err);
            let response = "La conexion no se ha podido establecer".to_string();
            Err(response)
        }
    }
}

/// Construye el vector de bytes que van a representar al paquete Publish y los devuelve
fn build_bytes_for_subscribe() -> Vec<u8> {
    [0x82, 0x09, 0x00, 0x0A, 0x00, 0x04, 0x54, 0x45, 0x4D, 0x50, 0x00].to_vec()
}

pub fn send_subscribe(mut stream: TcpStream) -> Result<(), String> {
    let subscribe_bytes = build_bytes_for_subscribe();
    if let Ok(_stream) = stream.write(&subscribe_bytes){
        Ok(()) 
    }else{
        Err("Error subscribing".to_string())
    }
}

fn random_value() -> [u8; 2] {
    let unit = rand::thread_rng().gen_range(48..57);
    let ten = rand::thread_rng().gen_range(48..53);
    [ten, unit]
}

/// Construye el vector de bytes que van a representar al paquete Publish y los devuelve
fn build_bytes_for_publish(ten: u8, unit: u8) -> Vec<u8> {
    [0x30, 0x08, 0x00, 0x04, 0x54, 0x45, 0x4D, 0x50, ten, unit].to_vec()
}

pub fn send_publish(mut stream: TcpStream, ten: u8, unit: u8) -> Result<(), String> {
    let publish_bytes = build_bytes_for_publish(ten, unit);
    if let Err(_err) = stream.write(&publish_bytes) {
        return Err("Error publishing".to_string());
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let stream = connect_to_server()?;
    if let Ok(stream_clone) = stream.try_clone() {
        send_subscribe(stream_clone)?;
    }
    thread::sleep(Duration::from_secs(1));
    loop {
        thread::sleep(Duration::from_secs(5));
        let [ten, unit]: [u8; 2] = random_value();
        if let Ok(stream_clone) = stream.try_clone() {
            send_publish(stream_clone, ten, unit)?;
        }
    }
}
