pub fn build_bytes_for_connect(user: String, password: String, id_client: String) -> Vec<u8> {
    let mut flags: u8 = 0x00;
    let mut bytes = vec![
        0x10, //Packet ID
        //0x00, Remaining Length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // Variable Header
        0x04, // Protocol
        0x00, //Flags
        0x00, 0x0B, //Keep Alive
    ];
    add_client_id_bytes(id_client, &mut bytes);
    add_username_bytes(user, &mut flags, &mut bytes);
    add_password_bytes(password, &mut flags, &mut bytes);
    bytes[8] = flags;
    let length = bytes.len();
    bytes.insert(1, (length - 1) as u8);
    bytes
}

fn add_client_id_bytes(id_client: String, bytes: &mut Vec<u8>) {
    if !id_client.is_empty() {
        let id_length = id_client.len();
        let mut id_client_in_bytes = id_client.as_bytes().to_vec();
        bytes.push(0x00);
        bytes.push(id_length as u8);
        bytes.append(&mut id_client_in_bytes);
    } else {
        bytes.append(&mut vec![0x00, 0x02, 0x00, 0x00]);
    }
}

fn add_password_bytes(password: String, flags: &mut u8, bytes: &mut Vec<u8>) {
    if !password.is_empty() {
        *flags |= 0b01000000;
        let password_length = password.len();
        let mut password_in_bytes = password.as_bytes().to_vec();
        bytes.push(0x00);
        bytes.push(password_length as u8);
        bytes.append(&mut password_in_bytes);
    }
}

fn add_username_bytes(user: String, flags: &mut u8, bytes: &mut Vec<u8>) {
    if !user.is_empty() {
        *flags |= 0b10000000;
        let user_length = user.len();
        let mut user_in_bytes = user.as_bytes().to_vec();
        bytes.push(0x00);
        bytes.push(user_length as u8);
        bytes.append(&mut user_in_bytes);
    }
}

pub fn build_bytes_for_suscribe(topic: String, is_qos_0: bool) -> Vec<u8> {
    let mut bytes = vec![
        //0x82 packet type
        //0x08, remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
    ];
    add_suscribe_packet_type(is_qos_0, &mut bytes);
    add_topic_bytes(topic, &mut bytes);
    add_qos_byte(is_qos_0, &mut bytes);
    let length = bytes.len();
    bytes.insert(1, (length - 1) as u8);
    bytes
}

pub fn build_bytes_for_unsubscribe(topic: String) -> Vec<u8> {
    let mut bytes = vec![0xA0, 0x00, 0x0A];
    add_topic_bytes(topic, &mut bytes);
    let length = bytes.len();
    bytes.insert(1, (length - 1) as u8);
    bytes
}

fn add_qos_byte(is_qos_0: bool, bytes: &mut Vec<u8>) {
    if !is_qos_0 {
        bytes.push(0x01);
    } else {
        bytes.push(0x00);
    }
}

fn add_suscribe_packet_type(is_qos_0: bool, bytes: &mut Vec<u8>) {
    if is_qos_0 {
        bytes.insert(0, 0x80)
    } else {
        bytes.insert(0, 0x82)
    }
}

fn add_topic_bytes(topic: String, bytes: &mut Vec<u8>) {
    if !topic.is_empty() {
        let topic_length = topic.len();
        let mut topic_in_bytes = topic.as_bytes().to_vec();
        bytes.push(0x00);
        bytes.push(topic_length as u8);
        bytes.append(&mut topic_in_bytes);
    }
}

pub fn build_bytes_for_publish(topic: String, message: String, is_qos_0: bool) -> Vec<u8> {
    let mut bytes = vec![
        //0x32 Paquete publish QoS 1
        //0x09,  Remaining Length
    ];
    add_publish_packet_type(is_qos_0, &mut bytes);
    add_topic_bytes(topic, &mut bytes);
    add_packet_identifier_bytes(is_qos_0, &mut bytes);
    add_message_bytes(message, &mut bytes);
    let length = bytes.len();
    bytes.insert(1, (length - 1) as u8);
    bytes
}

fn add_packet_identifier_bytes(is_qos_0: bool, bytes: &mut Vec<u8>) {
    if !is_qos_0 {
        bytes.push(0x00);
        bytes.push(0x01);
    }
}

fn add_publish_packet_type(is_qos_0: bool, bytes: &mut Vec<u8>) {
    if is_qos_0 {
        bytes.insert(0, 0x30)
    } else {
        bytes.insert(0, 0x32)
    }
}

fn add_message_bytes(message: String, bytes: &mut Vec<u8>) {
    if !message.is_empty() {
        let message_length = message.len();
        let mut message_in_bytes = message.as_bytes().to_vec();
        bytes.push(0x00);
        bytes.push(message_length as u8);
        bytes.append(&mut message_in_bytes);
    }
}
