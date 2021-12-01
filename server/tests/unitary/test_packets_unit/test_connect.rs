#[cfg(test)]
mod tests {
    use server::helper::user_manager::UserManager;
    use server::paquetes::connect::Connect;
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::{thread, time};

    fn setup() {
        match TcpListener::bind("0.0.0.0:1883") {
            Ok(listener) => {
                thread::spawn(move || for _i in listener.incoming() {});
                thread::sleep(time::Duration::from_millis(100));
            }
            Err(_) => {}
        }
    }

    #[test]
    fn create_connect_packet_and_get_client_id() {
        setup();
        let stream = TcpStream::connect("localhost:1883").unwrap();
        let mut user_manager = UserManager::new();
        let connect_bytes = [
            0x10, // packet type
            0x12, // remaining length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
            0x04, // protocol name
            0x00, // flags
            0x00, 0x0B, // keep alive
            0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, // client identifier
        ];
        let connect = Connect::init(&connect_bytes, &stream, &mut user_manager);

        assert_eq!(connect.get_client_id(), "ALTEGO".to_owned());
        // assert_eq!(connect.get_keep_alive(), Some(0x0B));
    }

    #[test]
    fn create_connect_packet_and_verify_correct_add_to_user_manager() {
        setup();
        let stream = TcpStream::connect("localhost:1883").unwrap();
        let mut user_manager = UserManager::new();
        let connect_bytes = [
            0x10, // packet type
            0x12, // remaining length
            0x00, 0x04, 0x4D, 0x15, 0x54, 0x54, //mqtt
            0x04, // protocol name
            0x00, // flags
            0x00, 0x0B, // keep alive
            0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, // client identifier
        ];
        let _connect = Connect::init(&connect_bytes, &stream, &mut user_manager);

        if let Some(_user) = user_manager.find_user("ALTEGO".to_owned()) {
            assert_eq!(1, 1);
        } else {
            assert_eq!(0, 1);
        }
    }
}
