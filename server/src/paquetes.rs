pub trait Paquetes {
    fn new() -> Box<dyn Paquetes> where Self: Sized;
    fn get_type(&self) -> String;
}