pub trait Paquetes {
    fn init(bytes: &[u8]) -> Box<dyn Paquetes>
    where
        Self: Sized;
    fn get_type(&self) -> String;
    fn get_remaining_length(&self) -> usize;
    fn get_status_code(self) -> u8;
}
