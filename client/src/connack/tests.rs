use crate::connack::Connack;

#[test]
fn create_connack() {
    let first_bytes = [
        0x20, // Paquete CONNACK
        0x02, // Remaining Length - 2 para paquete CONNACK, ya que no tiene payload
        0x01,
        0x00,
    ];

    let first_connack_packet = Connack::init(&first_bytes);
    assert_eq!(2,3)
}
