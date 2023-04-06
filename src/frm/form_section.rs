use std::io;

pub struct FormSection<B> {
    buffer: B,
}

impl<B: AsRef<[u8]>> FormSection<B> {
    pub fn new(buffer: B) -> io::Result<FormSection<B>> {
        if buffer.as_ref().len() != 286 {
            Err(io::Error::new(io::ErrorKind::Other, format!("FormSection len {}!=286", buffer.as_ref().len())))
        } else {
            Ok(Self { buffer })
        }
    }
}

impl<B: AsRef<[u8]>> FormSection<B> {
    /// 表中字段数
    pub fn column_count(&self) -> u16 {
        u16::from_be_bytes([self.buffer.as_ref()[258], self.buffer.as_ref()[259]])
    }
    /// screen section 长度
    pub fn screens_length(&self) -> u16 {
        u16::from_be_bytes([self.buffer.as_ref()[260], self.buffer.as_ref()[261]])
    }
    /// 允许为 null 的字段数量
    pub fn null_columns(&self) -> u16 {
        u16::from_be_bytes([self.buffer.as_ref()[282], self.buffer.as_ref()[283]])
    }
}