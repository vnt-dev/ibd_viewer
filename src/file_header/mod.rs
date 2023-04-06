use std::fmt;
use std::fmt::Formatter;

use crate::file_header::page_type::PageType;

pub mod page_type;

/// 名称   |长度|描述
/// ------|:------|:------
/// FIL_PAGE_SPACE_OR_CHKSUM     | 4 |页的校验和 (checksum)
/// FIL_PAGE_OFFSET	|4|	页号 (每个页的都不同)
/// FIL_PAGE_PREV	|4|	上一个页的页号
/// FIL_PAGE_NEXT	|4|	下一个页的页号
/// FIL_PAGE_LSN	|8|	页面被最后修改时对应的日志序列位置 (Log Sequence Number)
/// FIL_PAGE_TYPE	|2|	该页的类型
/// FIL_PAGE_FILE_FLUSH_LSN	|8|	仅在系统表空间的一个页中定义，代表文件至少被刷新到了对应的 LSN 值
/// FIL_PAGE_ARCH_LOG_NO_OR_SPACE_ID	|4|	页属于哪个表空间
///
pub struct FileHeader<B> {
    buffer: B,
}

impl<B: AsRef<[u8]>> FileHeader<B> {
    /// 固定38位
    pub fn new(buffer: B) -> FileHeader<B> {
        assert_eq!(
            buffer.as_ref().len(),
            38,
            "RecordHeader len {}!= 38",
            buffer.as_ref().len()
        );
        Self { buffer }
    }
}

impl<B: AsRef<[u8]>> FileHeader<B> {
    pub fn check_sum(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[..4].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    pub fn offset(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[4..8].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    pub fn prev(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[8..12].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    pub fn next(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[12..16].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    pub fn lsn(&self) -> u64 {
        let tmp: [u8; 8] = self.buffer.as_ref()[16..24].try_into().unwrap();
        u64::from_be_bytes(tmp)
    }
    pub fn page_type(&self) -> PageType {
        let tmp: [u8; 2] = self.buffer.as_ref()[24..26].try_into().unwrap();
        PageType::from(u16::from_be_bytes(tmp))
    }
    pub fn flush_lsn(&self) -> u64 {
        let tmp: [u8; 8] = self.buffer.as_ref()[26..34].try_into().unwrap();
        u64::from_be_bytes(tmp)
    }
    pub fn space_id(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[34..38].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
}

impl<B: AsRef<[u8]>> fmt::Display for FileHeader<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "FileHeader")?;
        writeln!(
            f,
            " check_sum:{} ({})",
            self.check_sum(),
            hex::encode(&self.buffer.as_ref()[..4])
        )?;
        writeln!(
            f,
            " offset:{} ({})",
            self.offset(),
            hex::encode(&self.buffer.as_ref()[4..8])
        )?;
        writeln!(
            f,
            " prev:{} ({})",
            self.prev(),
            hex::encode(&self.buffer.as_ref()[8..12])
        )?;
        writeln!(
            f,
            " next:{} ({})",
            self.next(),
            hex::encode(&self.buffer.as_ref()[12..16])
        )?;
        writeln!(
            f,
            " lsn:{} ({})",
            self.lsn(),
            hex::encode(&self.buffer.as_ref()[16..24])
        )?;
        writeln!(
            f,
            " page_type:{:?} ({})",
            self.page_type(),
            hex::encode(&self.buffer.as_ref()[24..26])
        )?;
        writeln!(
            f,
            " flush_lsn:{} ({})",
            self.flush_lsn(),
            hex::encode(&self.buffer.as_ref()[26..34])
        )?;
        writeln!(
            f,
            " space_id:{} ({})",
            self.space_id(),
            hex::encode(&self.buffer.as_ref()[34..38])
        )
    }
}

impl<B: AsRef<[u8]>> fmt::Debug for FileHeader<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileHeader")
            .field("check_sum", &self.check_sum())
            .field("offset", &self.offset())
            .field("prev", &self.prev())
            .field("next", &self.next())
            .field("lsn", &self.lsn())
            .field("page_type", &self.page_type())
            .field("flush_lsn", &self.flush_lsn())
            .field("space_id", &self.space_id())
            .finish()
    }
}
