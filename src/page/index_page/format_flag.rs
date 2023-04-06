#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum RecordFormat {
    Redundant = 0x00,
    Compact = 0x01,
}

impl From<u8> for RecordFormat {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
