pub struct IndexSection<B> {
    buffer: B,
}

impl<B: AsRef<[u8]>> IndexSection<B> {
    /// 索引数量
    pub fn key_count(&self) -> u16 {
        if self.buffer.as_ref()[0] & 0x80 == 0x80 {
            return (self.buffer.as_ref()[0] & 0x7f) as u16 + (self.buffer.as_ref()[1] as u16) << 7;
        }
        self.buffer.as_ref()[0] as u16
    }
    /// 索引中字段数量
    pub fn key_parts(&self) -> u16 {
        if self.buffer.as_ref()[0] & 0x80 == 0x80 {
            return u16::from_be_bytes([self.buffer.as_ref()[2], self.buffer.as_ref()[3]]);
        }
        self.buffer.as_ref()[1] as u16
    }
    /// 获取索引
    pub fn keys(&self) {

    }
}

pub struct IndexBlock<B> {
    buffer: B,
}
impl<B: AsRef<[u8]>> IndexBlock<B> {
    /// 索引标识
    pub fn flag(&self)->u16{
        u16::from_be_bytes([self.buffer.as_ref()[0], self.buffer.as_ref()[1]])
    }
    /// 索引中用户自定义字段数量
    pub fn user_defined_key_parts(&self)->u8{
        self.buffer.as_ref()[2]
    }

}
pub struct IndexBlockPart<B> {
    buffer: B,
}
impl<B: AsRef<[u8]>> IndexBlockPart<B> {

}

