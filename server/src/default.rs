use crate::paquetes::Paquetes;

pub struct Default {}

// Implement the `Animal` trait for `Cow`.
impl Paquetes for Default {
    fn new() -> Box<dyn Paquetes> {
        Box::new(Default {})
    }
    fn get_type(&self) -> String {
        "default".to_owned()
    }
}