use std::fmt;
use std::fmt::Formatter;

use crate::page::index_page::direction::Direction;
use crate::page::index_page::format_flag::RecordFormat;
use crate::page::FilePointer;

/// 名称             |长度|描述
/// ------------------|:--------------------|:------------------
/// PAGE_N_DIR_SLOTS |2|在页目录中的槽数量
/// PAGE_HEAP_TOP |2| 还未使用的空间最小地址，也就是说从该地址之后就是 Free Space
/// PAGE_N_HEAP&FORMAT_FLAG	|2|	本页中的记录的数量（包括最小和最大记录以及标记为删除的记录），第15位是行模式，0:Redundant,1:COMPACT模式,
/// PAGE_FREE	|2|	第一个已经标记为删除的记录地址（各个已删除的记录通过 next_record 也会组成一个单链表，这个单链表中的记录可以被重新利用）
/// PAGE_GARBAGE |2|	已删除的字节数，行记录结构中delete_flag为1的记录大小总数
/// PAGE_LAST_INSERT	|2|	最后插入记录的位置
/// PAGE_DIRECTION	|2|	记录插入的方向 (新插入记录的主键值比上一条记录的主键值大，插入方向就是右边，反之则是左边) 取值为 0x02 右 0x01 左  0x05 无序
/// PAGE_N_DIRECTION	|2|	一个方向连续插入的记录数量
/// PAGE_N_RECS	|2|	该页中记录的数量（不包括最小和最大记录以及被标记为删除的记录）
/// PAGE_MAX_TRX_ID	|8|	修改当前页的最大事务ID，该值仅在二级索引中定义
/// PAGE_LEVEL	|2|	当前页在索引树中的位置，高度
/// PAGE_INDEX_ID	|8|	索引ID，表示当前页属于哪个索引
pub struct IndexHeader<B> {
    buffer: B,
}

impl<B: AsRef<[u8]>> IndexHeader<B> {
    /// 固定56
    pub fn new(buffer: B) -> IndexHeader<B> {
        assert_eq!(
            buffer.as_ref().len(),
            36,
            "RecordHeader len {}!= 36",
            buffer.as_ref().len()
        );
        Self { buffer }
    }
}

impl<B: AsRef<[u8]>> IndexHeader<B> {
    pub fn slots(&self) -> u16 {
        let tmp: [u8; 2] = self.buffer.as_ref()[..2].try_into().unwrap();
        u16::from_be_bytes(tmp)
    }
    pub fn heap_top(&self) -> u16 {
        let tmp: [u8; 2] = self.buffer.as_ref()[2..4].try_into().unwrap();
        u16::from_be_bytes(tmp)
    }
    pub fn heap_num(&self) -> u16 {
        let tmp: [u8; 2] = self.buffer.as_ref()[4..6].try_into().unwrap();
        u16::from_be_bytes(tmp) & 0x7FFF
    }
    pub fn format_flag(&self) -> RecordFormat {
        RecordFormat::from(self.buffer.as_ref()[4] >> 7)
    }
    pub fn free(&self) -> u16 {
        let tmp: [u8; 2] = self.buffer.as_ref()[6..8].try_into().unwrap();
        u16::from_be_bytes(tmp)
    }
    pub fn garbage(&self) -> u16 {
        let tmp: [u8; 2] = self.buffer.as_ref()[8..10].try_into().unwrap();
        u16::from_be_bytes(tmp)
    }
    pub fn last_insert(&self) -> u16 {
        let tmp: [u8; 2] = self.buffer.as_ref()[10..12].try_into().unwrap();
        u16::from_be_bytes(tmp)
    }
    pub fn direction(&self) -> Direction {
        let tmp: [u8; 2] = self.buffer.as_ref()[12..14].try_into().unwrap();
        Direction::from(u16::from_be_bytes(tmp))
    }
    pub fn direction_num(&self) -> u16 {
        let tmp: [u8; 2] = self.buffer.as_ref()[14..16].try_into().unwrap();
        u16::from_be_bytes(tmp)
    }
    pub fn recs(&self) -> u16 {
        let tmp: [u8; 2] = self.buffer.as_ref()[16..18].try_into().unwrap();
        u16::from_be_bytes(tmp)
    }
    pub fn max_trx_id(&self) -> u64 {
        let tmp: [u8; 8] = self.buffer.as_ref()[18..26].try_into().unwrap();
        u64::from_be_bytes(tmp)
    }
    pub fn level(&self) -> u16 {
        let tmp: [u8; 2] = self.buffer.as_ref()[26..28].try_into().unwrap();
        u16::from_be_bytes(tmp)
    }
    pub fn index_id(&self) -> u64 {
        let tmp: [u8; 8] = self.buffer.as_ref()[28..36].try_into().unwrap();
        u64::from_be_bytes(tmp)
    }
}

impl<B: AsRef<[u8]>> fmt::Display for IndexHeader<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "IndexPageHeader")?;
        writeln!(
            f,
            " slots:{} ({})",
            self.slots(),
            hex::encode(&self.buffer.as_ref()[..2])
        )?;
        writeln!(
            f,
            " heap_top:{} ({})",
            self.heap_top(),
            hex::encode(&self.buffer.as_ref()[2..4])
        )?;
        writeln!(
            f,
            " format_flag:{:?} ({} >> 7)",
            self.format_flag(),
            hex::encode(&[self.buffer.as_ref()[4]])
        )?;
        writeln!(
            f,
            " heap_num:{} ({})",
            self.heap_num(),
            hex::encode(&self.buffer.as_ref()[4..6])
        )?;
        writeln!(
            f,
            " free:{} ({})",
            self.free(),
            hex::encode(&self.buffer.as_ref()[6..8])
        )?;
        writeln!(
            f,
            " garbage:{} ({})",
            self.garbage(),
            hex::encode(&self.buffer.as_ref()[8..10])
        )?;
        writeln!(
            f,
            " last_insert:{} ({})",
            self.last_insert(),
            hex::encode(&self.buffer.as_ref()[10..12])
        )?;
        writeln!(
            f,
            " direction:{:?} ({})",
            self.direction(),
            hex::encode(&self.buffer.as_ref()[12..14])
        )?;
        writeln!(
            f,
            " direction_num:{} ({})",
            self.direction_num(),
            hex::encode(&self.buffer.as_ref()[14..16])
        )?;
        writeln!(
            f,
            " recs:{} ({})",
            self.recs(),
            hex::encode(&self.buffer.as_ref()[16..18])
        )?;
        writeln!(
            f,
            " max_trx_id:{} ({})",
            self.max_trx_id(),
            hex::encode(&self.buffer.as_ref()[18..26])
        )?;
        writeln!(
            f,
            " level:{} ({})",
            self.level(),
            hex::encode(&self.buffer.as_ref()[26..28])
        )?;
        writeln!(
            f,
            " index_id:{} ({})",
            self.index_id(),
            hex::encode(&self.buffer.as_ref()[28..36])
        )
    }
}

impl<B: AsRef<[u8]>> fmt::Debug for IndexHeader<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IndexPageHeader")
            .field("slots", &self.slots())
            .field("heap_top", &self.heap_top())
            .field("format_flag", &self.format_flag())
            .field("heap_num", &self.heap_num())
            .field("free", &self.free())
            .field("garbage", &self.garbage())
            .field("last_insert", &self.last_insert())
            .field("direction", &self.direction())
            .field("direction_num", &self.direction_num())
            .field("recs", &self.recs())
            .field("max_trx_id", &self.max_trx_id())
            .field("level", &self.level())
            .field("index_id", &self.index_id())
            .finish()
    }
}

/// 仅在根节点定义
/// 名称             |长度|描述
/// ------------------|:--------------------|:------------------
/// LEAF_SPACE_ID	|4|	叶子节点所在段的space id
/// LEAF_PAGE_NUM	|4|	叶子节点所在段的Inode页号
/// LEAF_OFFSET	|2|	叶子节点所在段内Inode页的偏移量
/// INTERNAL_SPACE_ID	|4|	非叶子节点所在段的space id
/// INTERNAL_PAGE_NUM	|4|	非叶子节点所在段的Inode页号
/// INTERNAL_OFFSET	|2|	非叶子节点所在段内Inode页的偏移量
pub struct FSegHeader<B> {
    buffer: B,
}

impl<B: AsRef<[u8]>> FSegHeader<B> {
    /// 固定20
    pub fn new(buffer: B) -> FSegHeader<B> {
        assert_eq!(
            buffer.as_ref().len(),
            20,
            "RecordHeader len {}!= 20",
            buffer.as_ref().len()
        );
        Self { buffer }
    }
    pub fn leaf_space_id(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[..4].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    pub fn leaf_pointer(&self) -> FilePointer {
        FilePointer::new(&self.buffer.as_ref()[4..10])
    }
    pub fn internal_space_id(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[10..14].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    pub fn internal_pointer(&self) -> FilePointer {
        FilePointer::new(&self.buffer.as_ref()[14..20])
    }
}

impl<B: AsRef<[u8]>> fmt::Display for FSegHeader<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "FSegHeader")?;
        writeln!(
            f,
            " leaf_space_id:{} ({})",
            self.leaf_space_id(),
            hex::encode(&self.buffer.as_ref()[..4])
        )?;
        writeln!(
            f,
            " leaf_pointer:{:?} ({})",
            self.leaf_pointer(),
            hex::encode(&self.buffer.as_ref()[4..10])
        )?;
        writeln!(
            f,
            " internal_space_id:{} ({})",
            self.internal_space_id(),
            hex::encode(&self.buffer.as_ref()[10..14])
        )?;
        writeln!(
            f,
            " internal_pointer:{:?} ({})",
            self.internal_pointer(),
            hex::encode(&self.buffer.as_ref()[14..20])
        )
    }
}

impl<B: AsRef<[u8]>> fmt::Debug for FSegHeader<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FSegHeader")
            .field("leaf_space_id", &self.leaf_space_id())
            .field("leaf_pointer", &self.leaf_pointer())
            .field("internal_space_id", &self.internal_space_id())
            .field("internal_pointer", &self.internal_pointer())
            .finish()
    }
}
