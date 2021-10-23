use crate::paquetes::Paquetes;

pub struct Connect {}

// Implement the `Animal` trait for `Sheep`.
impl Paquetes for Connect {
    fn init() -> Box<dyn Paquetes> {
        Box::new(Connect {})
    }
    fn get_type(&self) -> String {
        "connect".to_owned()
    }
}
