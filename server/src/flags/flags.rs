pub trait Flags {
    fn new(bytes: &u8) -> Box<dyn Flags>
    where
        Self: Sized;
    fn get_username_flag(&self) -> bool;
    fn get_password_flag(&self) -> bool;
    fn get_will_retain_flag(&self) -> bool;
    fn get_will_flag(&self) -> bool;
    fn get_clean_session_flag(&self) -> bool;
    fn get_will_qos_flag(&self) -> u8;
}
