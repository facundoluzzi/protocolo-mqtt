/// Construye el vector de bytes que van a representar al paquete Subscribe y los devuelve
pub fn build_bytes_for_suscribe(list_of_topics: Vec<(String, bool)>) -> Vec<u8> {
    let mut bytes = vec![
        //0x82 packet type
        //0x08, remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
    ];
    add_suscribe_packet_type(&mut bytes);
    add_bytes_of_topic_list(list_of_topics, &mut bytes);
    let length = bytes.len();
    bytes.insert(1, (length - 1) as u8);
    bytes
}

/// Construye el vector de bytes que van a representar al paquete Unsubscribe y los devuelve
pub fn build_bytes_for_unsubscribe(topic: String) -> Vec<u8> {
    let mut bytes = vec![0xA2, 0x00, 0x00, 0x0A];
    let mut topic_as_bytes = topic.as_bytes().to_vec();
    bytes.push(0x00);
    bytes.push(topic_as_bytes.len() as u8);
    bytes.append(&mut topic_as_bytes);
    let length = bytes.len();
    bytes[1] = (length - 2) as u8;
    bytes
}

/// Agrega al vector de bytes, los bytes correspondientes al packet type
fn add_suscribe_packet_type(bytes: &mut Vec<u8>) {
    bytes.insert(0, 0x82)
}

/// Agrega al vector de bytes, los bytes que corresponden a todos los topics que se quieren suscribir, presentes en el paquete subscribe
fn add_bytes_of_topic_list(topic_list: Vec<(String, bool)>, bytes: &mut Vec<u8>) {
    if !topic_list.is_empty() {
        for (topic, is_qos_0) in topic_list {
            let topic_length = topic.len();
            let mut topic_in_bytes = topic.as_bytes().to_vec();
            bytes.push(0x00);
            bytes.push(topic_length as u8);
            bytes.append(&mut topic_in_bytes);
            if is_qos_0 {
                bytes.push(0x00);
            } else {
                bytes.push(0x01);
            }
        }
    }
}

/// Agrega al vector de bytes, los bytes correspondientes al topico
fn add_topic_bytes_for_publish(topic: String, bytes: &mut Vec<u8>) {
    if !topic.is_empty() {
        let topic_length = topic.len();
        let mut topic_in_bytes = topic.as_bytes().to_vec();
        bytes.push(0x00);
        bytes.push(topic_length as u8);
        bytes.append(&mut topic_in_bytes);
    }
}

/// Construye el vector de bytes que van a representar al paquete Publish y los devuelve
pub fn build_bytes_for_publish(
    topic: String,
    message: String,
    is_qos_0: bool,
    retain_is_active: bool,
) -> Vec<u8> {
    let mut bytes = vec![
        //0x32 Paquete publish QoS 1
        //0x09,  Remaining Length
    ];
    add_publish_packet_type(is_qos_0, retain_is_active, &mut bytes);
    add_topic_bytes_for_publish(topic, &mut bytes);
    add_packet_identifier_bytes(is_qos_0, &mut bytes);
    add_message_bytes(message, &mut bytes);
    let length = bytes.len();
    bytes.insert(1, (length - 1) as u8);
    bytes
}

/// Agrega al vector de bytes, los bytes correspondientes al packet ID
fn add_packet_identifier_bytes(is_qos_0: bool, bytes: &mut Vec<u8>) {
    if !is_qos_0 {
        bytes.push(0x00);
        bytes.push(0x01);
    }
}

/// Agrega al vector de bytes, los bytes correspondientes al packet type
fn add_publish_packet_type(is_qos_0: bool, retain_is_active: bool, bytes: &mut Vec<u8>) {
    if is_qos_0 && retain_is_active {
        bytes.insert(0, 0x31)
    } else if is_qos_0 && !retain_is_active {
        bytes.insert(0, 0x30)
    } else if !is_qos_0 && retain_is_active {
        bytes.insert(0, 0x33)
    } else {
        bytes.insert(0, 0x32)
    }
}

/// Agrega al vector de bytes, los bytes correspondientes al mensaje
fn add_message_bytes(message: String, bytes: &mut Vec<u8>) {
    if !message.is_empty() {
        let mut message_in_bytes = message.as_bytes().to_vec();
        bytes.append(&mut message_in_bytes);
    }
}

/// Construye el vector de bytes que van a representar al paquete Disconnect y los devuelve
pub fn build_bytes_for_disconnect() -> Vec<u8> {
    vec![0xE0, 0x00]
}
