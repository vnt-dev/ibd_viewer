use std::fmt;
use std::fmt::Formatter;

pub const NEW_HEAD_LEN: usize = 5;

/// 名称	|大小（单位：bit）|	描述
/// ------|:------|:------
/// 预留位1	|1|	没有使用
/// 预留位2	|1|	没有使用
/// delete_mask	|1|	标记该记录是否被删除 1表示已删除
/// min_rec_mask	|1|	标记该记录是否为B+树的非叶子节点中的最小记录
/// n_owned	|4|	表示当前槽管理的记录数
/// heap_no	|13|	表示当前记录在记录堆的位置信息
/// record_type	|3|	表示当前记录的类型，0表示普通记录，1表示B+树非叶节点记录，2表示最小记录，3表示最大记录
/// next_record	|16|	表示下一条记录的相对位置
pub struct RecordHeader<B> {
    buffer: B,
}

impl<B: AsRef<[u8]>> RecordHeader<B> {
    /// 固定5
    pub fn new(buffer: B) -> RecordHeader<B> {
        assert_eq!(
            buffer.as_ref().len(),
            5,
            "RecordHeader len {}!= 5",
            buffer.as_ref().len()
        );
        Self { buffer }
    }
}

impl<B: AsRef<[u8]>> RecordHeader<B> {
    pub fn buf(&self) -> &[u8] {
        self.buffer.as_ref()
    }

    pub fn not_used(&self) -> u8 {
        self.buffer.as_ref()[0] >> 6
    }
    pub fn delete_mask(&self) -> u8 {
        (self.buffer.as_ref()[0] >> 5) & 0b1
    }
    pub fn min_rec_mask(&self) -> u8 {
        (self.buffer.as_ref()[0] >> 4) & 0b1
    }
    pub fn n_owned(&self) -> u8 {
        self.buffer.as_ref()[0] & 0x0F
    }
    pub fn heap_no(&self) -> u16 {
        let tmp: [u8; 2] = self.buffer.as_ref()[1..3].try_into().unwrap();
        u16::from_be_bytes(tmp) >> 3
    }
    pub fn record_type(&self) -> u8 {
        self.buffer.as_ref()[2] & 0x07
    }
    pub fn next_record(&self) -> i16 {
        let tmp: [u8; 2] = self.buffer.as_ref()[3..5].try_into().unwrap();
        i16::from_be_bytes(tmp)
    }
}
impl<B: AsRef<[u8]>> fmt::Display for RecordHeader<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}>>6 /", hex::encode(&[self.buffer.as_ref()[0]]))?;
        write!(f, " ({}>>5)&0b1 /", hex::encode(&[self.buffer.as_ref()[0]]))?;
        write!(f, " ({}>>4)&0b1 /", hex::encode(&[self.buffer.as_ref()[0]]))?;
        write!(f, " {}&0x0F /", hex::encode(&[self.buffer.as_ref()[0]]))?;
        write!(f, " {}>>3 /", hex::encode(&self.buffer.as_ref()[1..3]))?;
        write!(f, " {}&0x07 /", hex::encode(&[self.buffer.as_ref()[2]]))?;
        write!(f, " {} /", hex::encode(&self.buffer.as_ref()[3..5]))
    }
}

impl<B: AsRef<[u8]>> fmt::Debug for RecordHeader<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecordHeader")
            .field("not_used", &self.not_used())
            .field("delete_mask", &self.delete_mask())
            .field("min_rec_mask", &self.min_rec_mask())
            .field("n_owned", &self.n_owned())
            .field("heap_no", &self.heap_no())
            .field("record_type", &self.record_type())
            .field("next_record", &self.next_record())
            .finish()
    }
}
