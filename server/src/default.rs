use crate::paquetes::Paquetes;

pub struct Default {}

// Implement the `Animal` trait for `Cow`.
impl Paquetes for Default {
    fn save_remaining_length(&mut self, bytes: &[u8]) -> Result<String, String> {
        Ok("".to_string())
    }

    fn get_remaining_length(&self) -> usize {
        0
    }

    fn init(bytes: &[u8]) -> Box<dyn Paquetes> {
        Box::new(Default {})
    }
    fn get_type(&self) -> String {
        "default".to_owned()
    }
}
