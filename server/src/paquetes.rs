pub trait Paquetes {
    fn init() -> Box<dyn Paquetes>
    where
        Self: Sized;
    fn get_type(&self) -> String;
}
