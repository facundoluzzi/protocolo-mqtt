use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::io::Read;
use cliente_dos::helper::stream::stream_handler::Stream;
use cliente_dos::types::StreamType;
use cliente_dos::helper::stream::stream_handler::StreamAction::WriteStream;
use cliente_dos::helper::stream::stream_handler::StreamAction::ReadStream;
use std::sync::mpsc::Sender;
use std::sync::mpsc;

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

fn send_connect(sender_stream: Sender<StreamType>) -> Result<(), String> {
    let connect_bytes = build_bytes_for_connect();
    if sender_stream
        .send((WriteStream, Some(connect_bytes), None))
        .is_err()
    {
        return Err("Error enviando el paquete connect".to_string());
    }

    Ok(())
}

fn connect() -> Result<Sender<StreamType>, String> {
    // channel
    match TcpStream::connect("localhost:1883") {
        Ok(stream) => {
            if let Ok(sender_stream) = Stream::init(stream) {
                send_connect(sender_stream.clone()).unwrap();
                start_to_read(sender_stream.clone());
                Ok(sender_stream)
            } else {
                Err("Error clonando inicializando el stream".to_string())
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

pub fn send_subscribe(sender_stream: Sender<StreamType>) -> Result<(), String> {
    let subscribe_bytes = build_bytes_for_subscribe();
    if sender_stream
        .send((WriteStream, Some(subscribe_bytes), None))
        .is_err()
    {
        return Err("Error enviando el paquete connect".to_string());
    }

    Ok(())
}

fn process_packet(packet: &[u8], sender_stream: Sender<StreamType>) ->  Result<(), String> {
    let first_byte = packet.get(0).unwrap();

    match first_byte {
        4 => {
            println!("llego temperatura");
            Ok(())
        },
        _ => {
            println!("llego otra cosa");
            Ok(())
        },
    }
}

fn is_empty_packet(packet: Vec<u8>) -> bool {
    !packet.into_iter().any(|element| element != 0)
}

fn start_to_read(
    sender_stream: Sender<StreamType>,
) {
    thread::spawn(move || {
        let (packet_sender, packet_receiver) = mpsc::channel::<Vec<u8>>();

        loop {
            let message_sent =
                sender_stream
                    .clone()
                    .send((ReadStream, None, Some(packet_sender.clone())));

            if let Err(_msg) = message_sent {
            } else if let Ok(packet) = packet_receiver.recv() {
                let empty_packet = is_empty_packet(packet.clone());
                if empty_packet {
                    break;
                }
                let packet_u8: &[u8] = &packet;
                if let Err(err) =
                    process_packet(packet_u8, sender_stream.clone())
                {
                    println!("err: {}", err);
                    break;
                }
            }
        }
    });
}

fn main() {
    // guardar los datos en memoria
    // y esperar por peticiones get para devolver todos los datos generados

    let sender_stream = connect();
    send_subscribe(sender_stream.unwrap().clone());
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

                            let response = "HTTP/1.1 200 OK\n\nasd";

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