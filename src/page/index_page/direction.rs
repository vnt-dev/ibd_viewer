/// 插入方向的枚举
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u16)]
pub enum Direction {
    Left = 0x0001,
    Right = 0x0002,
    NoDirection = 0x0005,
}

impl From<u16> for Direction {
    fn from(value: u16) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
