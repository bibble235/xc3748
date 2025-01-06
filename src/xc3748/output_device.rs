// Create a trait to writing to device
pub trait OutputDevice {
    fn write_data(&mut self, buf: &[u8]) -> ();
}
