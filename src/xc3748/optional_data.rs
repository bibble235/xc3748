#[derive(Debug, Clone, Copy)]
pub enum OptionalData {
    None,
    Data([u8; 2]),
}
