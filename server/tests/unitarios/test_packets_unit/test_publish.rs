#[cfg(test)]
mod tests {
    use server::paquetes::{publish::Publish};

    #[test]
    fn create_publish_packet_succesfully() {
        let bytes = [
            0x32, // Paquete publish 
            0x0A, // Remaining Length
            0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, // Topic
            0x00, 0x10, // Packet Identifier
        ];
        let publish = Publish::init(&bytes);
        let topic = publish.get_topic();
        assert_eq!(topic, "ALTEGO".to_owned());

    }
}
