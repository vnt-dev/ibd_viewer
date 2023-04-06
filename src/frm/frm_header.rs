pub struct FrmHeader<B> {
    buffer: B,
}

impl<B: AsRef<[u8]>> FrmHeader<B> {
    /// frm 文件版本号，表中不包含 varchar 字段时，值为 9
    /// 表中包含 varchar 字段时值为 10
    pub fn frm_version(&self) -> u8 {
        self.buffer.as_ref()[2]
    }
    /// 存储引擎类型，12 表示 InnoDB
    pub fn db_type(&self) -> u8 {
        self.buffer.as_ref()[3]
    }
    /// hybrid section 开始处的 Offset
    pub fn hybrid_section_offset(&self) -> u16 {
        u16::from_be_bytes([self.buffer.as_ref()[6], self.buffer.as_ref()[7]])
    }
    /// fields block 开始处的 Offset，按 4096 字节对齐
    pub fn fields_block_offset(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[10..14].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    /// index section 长度，如果 2 字节存储不下索引的长度，则 Offset 14 ~ 15 的所有位全部置为 1，
    /// 即 0xffff，然后把 index section 长度存储到 Offset 47 ~ 50 字节处
    /// 如果 2 字节能存储下索引的长度，则 Offset 14 ~ 15 处存储的值和 Offset 47 ~ 50 处存储的值相等
    pub fn index_section_len(&self) -> u32 {
        let tmp = u16::from_be_bytes([self.buffer.as_ref()[14], self.buffer.as_ref()[15]]);
        if tmp == 0xFFFF {
            let tmp: [u8; 4] = self.buffer.as_ref()[47..50].try_into().unwrap();
            return u32::from_be_bytes(tmp);
        }
        tmp as u32
    }
    /// index section 实际内容占用字节数（index section 长度大于 实际内容占用字节数）
    pub fn index_section_real_len(&self) -> u16 {
        u16::from_be_bytes([self.buffer.as_ref()[28], self.buffer.as_ref()[29]])
    }
    /// 字符集 ID
    pub fn character_set(&self) -> u8 {
        self.buffer.as_ref()[38]
    }
    /// 行类型，255：未定义、0：默认、1：FIXED、2：DYNAMIC、3：COMPRESSED、4：REDUNDANT、5：COMPACT
    pub fn row_format(&self) -> u8 {
        self.buffer.as_ref()[40]
    }
    /// MySQL 版本号
    pub fn mysql_version(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[51..54].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    pub fn extra_section_len(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[55..58].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
}