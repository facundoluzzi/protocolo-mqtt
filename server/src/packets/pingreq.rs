use crate::packets::packet_manager::PacketManager;
use crate::stream::stream_handler::StreamAction::WriteStream;

/// Procesa el pingreq y devuelve un pingresp, si todo es correcto.
pub fn send_response(packet_manager: &PacketManager) -> Result<(), String> {
    if packet_manager.is_disconnected() {
        Err("Client is not connected".to_string())
    } else {
        let sender_stream = packet_manager.get_sender_stream();
        let bytes_response = [0xE0, 0x00];
        let tuple_to_send = (WriteStream, Some(bytes_response.to_vec()), None, None);
        let sender_result = sender_stream.send(tuple_to_send);
        match sender_result {
            Err(err) => Err(err.to_string()),
            Ok(()) => Ok(()),
        }
    }
}
