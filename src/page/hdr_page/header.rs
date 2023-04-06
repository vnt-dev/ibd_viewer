use crate::page::ListBaseNode;
use std::fmt;

pub struct HdrFspHeader<B> {
    buffer: B,
}

impl<B: AsRef<[u8]>> HdrFspHeader<B> {
    pub fn new(buffer: B) -> HdrFspHeader<B> {
        assert_eq!(
            buffer.as_ref().len(),
            112,
            "RecordHeader len {}!= 112",
            buffer.as_ref().len()
        );
        Self { buffer }
    }
}

impl<B: AsRef<[u8]>> HdrFspHeader<B> {
    ///文件所属 space id
    pub fn space_id(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[..4].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    pub fn not_used(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[4..8].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    /// File Space 大小，单位为 page
    pub fn size(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[8..12].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    /// 当前尚未初始化的最小 page no，从该 page 往后的 page 都尚未加入到表空间的 FREE LIST 上，只是完成了分配而已
    pub fn free_limit(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[12..16].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    /// 当前 space 的一些 flag
    pub fn space_flags(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[16..20].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    /// FSP_FREE_FRAG 链表上所有 extent 已被使用的 page 数，用于快速计算该链表上可用空闲 page 数
    pub fn frag_n_used(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[20..24].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    /// extent list，extent 中所有 page 都未使用
    pub fn free(&self) -> ListBaseNode {
        ListBaseNode::new(&self.buffer.as_ref()[24..40])
    }
    /// extent list，extent 中部分 page 被使用
    pub fn free_frag(&self) -> ListBaseNode {
        ListBaseNode::new(&self.buffer.as_ref()[40..56])
    }
    /// extent list，extent 中全部 page 都被使用
    pub fn full_frag(&self) -> ListBaseNode {
        ListBaseNode::new(&self.buffer.as_ref()[56..72])
    }
    /// 下一个分配的 segment 的 id
    pub fn seg_id(&self) -> u64 {
        let tmp: [u8; 8] = self.buffer.as_ref()[72..80].try_into().unwrap();
        u64::from_be_bytes(tmp)
    }
    /// inode page list，page 中所有 inode entry 都被使用
    pub fn seg_inodes_full(&self) -> ListBaseNode {
        ListBaseNode::new(&self.buffer.as_ref()[80..96])
    }
    /// inode page list，page 中部分 inode entry 被使用
    pub fn seg_inodes_free(&self) -> ListBaseNode {
        ListBaseNode::new(&self.buffer.as_ref()[96..112])
    }
}

impl<B: AsRef<[u8]>> fmt::Debug for HdrFspHeader<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HdrFspHeader")
            .field("space_id", &self.space_id())
            .field("not_used", &self.not_used())
            .field("size", &self.size())
            .field("free_limit", &self.free_limit())
            .field("space_flags", &self.space_flags())
            .field("frag_n_used", &self.frag_n_used())
            .field("free", &self.free())
            .field("free_frag", &self.free_frag())
            .field("full_frag", &self.full_frag())
            .field("seg_id", &self.seg_id())
            .field("seg_inodes_full", &self.seg_inodes_full())
            .field("seg_inodes_free", &self.seg_inodes_free())
            .finish()
    }
}
