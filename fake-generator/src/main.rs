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
    stream.write(&connect_bytes).unwrap();
    Ok(())
}

pub fn connect_to_server() -> Result<TcpStream, String> {
    let address = "localhost:1883".to_owned();
    match TcpStream::connect(address) {
        Ok(stream) => {
            send_connect(stream.try_clone().unwrap()).unwrap();
            Ok(stream.try_clone().unwrap())
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
    stream.write(&subscribe_bytes).unwrap();
    Ok(())
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
    stream.write(&publish_bytes).unwrap();
    Ok(())
}

fn main() {
    let stream = connect_to_server().unwrap();
    send_subscribe(stream.try_clone().unwrap()).unwrap();
    thread::sleep(Duration::from_secs(1));
    loop {
        thread::sleep(Duration::from_secs(5));
        let [ten, unit]: [u8; 2] = random_value();
        send_publish(stream.try_clone().unwrap(), ten, unit).unwrap();
        println!("{}{}", ten, unit);
    }
}
