#[cfg(test)]
mod tests {
    use server::{paquetes::packet_manager::PacketManager};
    #[test]
    fn send_connect_should_select_connect_type() {
        let bytes = [
            0x10, 0x0E, 0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x04, 0x00, 0x00, 0x0B, 0x00, 0x02,
            0x00, 0x00,
        ];
        let connect_packet = PacketManager::get_control_packet_type(bytes[0]);
        assert_eq!(connect_packet, 1);
    }

    #[test]
    fn send_publish_should_select_publish_type() {
        let bytes = [
            0x30, 0x0A, 0x00, 0x05, 0x54, 0x4F, 0x50, 0x49, 0x43, 0x00, 0x06, 0x54,
        ];
        let publish_packet = PacketManager::get_control_packet_type(bytes[0]);
        assert_eq!(publish_packet, 3);
    }    

    #[test]
    fn send_subscribe_should_select_subscribe_type() {
        let bytes = [
            0x80, 0x0A, 0x00, 0x05, 0x54, 0x4F, 0x50, 0x49, 0x43, 0x00, 0x06, 0x54,
        ];
        let subscribe_packet = PacketManager::get_control_packet_type(bytes[0]);
        assert_eq!(subscribe_packet, 8);
    }    
}
