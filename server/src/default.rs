use crate::paquetes::Paquetes;

pub struct Default {}

// Implement the `Animal` trait for `Cow`.
impl Paquetes for Default {
    fn save_remaining_length(&mut self, _bytes: &[u8]) -> Result<usize, String> {
        Ok(0)
    }

    fn get_remaining_length(&self) -> usize {
        0
    }

    fn init(_bytes: &[u8]) -> Box<dyn Paquetes> {
        Box::new(Default {})
    }
    fn get_type(&self) -> String {
        "default".to_owned()
    }
}
