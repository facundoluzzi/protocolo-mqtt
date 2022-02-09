use crate::mpsc::Receiver;
use cliente_dos::enums::data_actions::AddData;
use cliente_dos::enums::data_actions::DataAction;
use cliente_dos::enums::data_actions::GetData;
use cliente_dos::helper::stream::stream_handler::Stream;
use cliente_dos::helper::stream::stream_handler::StreamAction::ReadStream;
use cliente_dos::helper::stream::stream_handler::StreamAction::WriteStream;
use cliente_dos::types::StreamType;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

fn build_bytes_for_connect() -> Vec<u8> {
    [
        0x10, 0x12, 0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x04, 0x00, 0x00, 0x00, 0x00, 0x06, 0x44,
        0x44, 0x50, 0x45, 0x47, 0x45,
    ]
    .to_vec()
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

fn connect(sender_to_save_event: Sender<DataAction>) -> Result<Sender<StreamType>, String> {
    // channel
    match TcpStream::connect("localhost:1883") {
        Ok(stream) => {
            if let Ok(sender_stream) = Stream::init(stream) {
                send_connect(sender_stream.clone())?;
                start_to_read(sender_stream.clone(), sender_to_save_event.clone());
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
    [
        0x82, 0x09, 0x00, 0x0A, 0x00, 0x04, 0x54, 0x45, 0x4D, 0x50, 0x00,
    ]
    .to_vec()
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

/// Construye el vector de bytes que van a representar al paquete Publish y los devuelve
fn build_bytes_for_disconnect() -> Vec<u8> {
    [0xE0, 0x00].to_vec()
}

pub fn send_disconnect(sender_stream: Sender<StreamType>) -> Result<(), String> {
    let disconnect_bytes = build_bytes_for_disconnect();
    if sender_stream
        .send((WriteStream, Some(disconnect_bytes), None))
        .is_err()
    {
        return Err("Error enviando el paquete connect".to_string());
    }

    Ok(())
}

fn process_packet(packet: &[u8], sender_to_save_event: Sender<DataAction>) -> Result<(), String> {
    match packet.get(0) {
        Some(first_byte) => match first_byte {
            48 => {
                let decena = packet[packet.len() - 2];
                let decimal = packet[packet.len() - 1];
                if let Ok(temperature) = String::from_utf8(vec![0x00, 0x02, decena, decimal]) {
                    let add_data = AddData::init(temperature[2..4].to_string());
                    if let Err(_err) = sender_to_save_event.send(DataAction::Add(add_data)) {
                        return Err("Unexpected error sending a message".to_string());
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        },
        None => Err("Incorrect packet".to_string()),
    }
}

fn is_empty_packet(packet: Vec<u8>) -> bool {
    !packet.into_iter().any(|element| element != 0)
}

fn start_to_read(sender_stream: Sender<StreamType>, sender_to_save_event: Sender<DataAction>) {
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
                if let Err(err) = process_packet(packet_u8, sender_to_save_event.clone()) {
                    println!("err: {}", err);
                    break;
                }
            }
        }
    });
}

fn connect_and_subscribe(sender_for_actions: Sender<DataAction>) -> Result<Sender<StreamType>, String> {
    if let Ok(sender_stream) = connect(sender_for_actions.clone()) {
        if let Err(_err) = send_subscribe(sender_stream.clone()) {
            return Err("Can not subscribe".to_string());
        } else {
            return Ok(sender_stream);
        }
    } else {
        return Err("Can not connect".to_string());
    }
}

fn spawn_data(receiver_for_actions: Receiver<DataAction>) {
    thread::spawn(move || {
        let mut vec: Vec<String> = Vec::new();
        for receive in receiver_for_actions {
            match receive {
                DataAction::Add(action) => {
                    vec.push(action.get_data());
                }
                DataAction::Get(action) => {
                    action.send_data(vec.clone());
                }
            }
        }
    });
}

fn disconnect(sender_stream: Sender<StreamType>) {
    loop {
        println!("Presione alguna letra para finalizar: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Error al leer de teclado");
        let input_as_bytes = input.as_bytes();
        if input_as_bytes.len() == 1 && input_as_bytes[0] == 10 {
            continue;
        }
        if let Err(_err) = send_disconnect(sender_stream.clone()) {
            println!("Unexpected error");
            return;
        }
        return;
    }
}

fn process_response(action: Sender<DataAction>) -> String {
    let (sender_to_get_data, receiver_data) = mpsc::channel::<Vec<String>>();
    let get_data = GetData::init(sender_to_get_data.clone());
    if let Err(_err) = action.send(DataAction::Get(get_data)) {
        println!("Unexpected error");
    }

    let data = match receiver_data.recv() {
        Ok(data) => data,
        Err(_) => Vec::new()
    };

    let response_text: String = data
        .into_iter()
        .map(|temp| {
            let mut temp_string = String::from("");
            temp_string += &temp.to_string();
            temp_string += "°C\n";
            temp_string.to_string()
        })
        .collect::<String>();

    "HTTP/1.1 200 OK\nAccess-Control-Allow-Origin: *\nAccess-Control-Allow-Header: text/plain\nContent-Type: text/plain\n\n".to_string() + &response_text
}

fn spawn_listener(sender_for_actions: Sender<DataAction>) {
    thread::spawn(move || {
        let listener = match TcpListener::bind("0.0.0.0:3000") {
            Ok(listener) => listener,
            Err(_err) => return,
        };
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let action = sender_for_actions.clone();
                    thread::spawn(move || {
                        let mut data = [0_u8; 400];
                        match stream.read(&mut data) {
                            Ok(_size) => {
                                if let Err(err) = stream.write_all(process_response(action.clone()).as_bytes()) {
                                    println!("{}", err);
                                }
                            }
                            Err(_err) => {
                                println!("Can not write");
                            }
                        }
                    });
                }
                Err(_err) => {
                    println!("Stream is not available");
                }
            }
        }
    });
}

fn main() {
    let (sender_for_actions, receiver_for_actions) = mpsc::channel::<DataAction>();
    let sender_stream = match connect_and_subscribe(sender_for_actions.clone()) {
        Ok(sender_stream) => sender_stream.clone(),
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    spawn_data(receiver_for_actions);
    spawn_listener(sender_for_actions.clone());
    disconnect(sender_stream.clone());
}
