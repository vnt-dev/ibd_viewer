use crate::page::ListNode;
use std::fmt;

pub struct XDesEntry<B> {
    buffer: B,
}

impl<B: AsRef<[u8]>> XDesEntry<B> {
    pub fn new(buffer: B) -> XDesEntry<B> {
        assert_eq!(
            buffer.as_ref().len(),
            40,
            "XdesEntry len {}!= 40",
            buffer.as_ref().len()
        );
        Self { buffer }
    }
}

impl<B: AsRef<[u8]>> XDesEntry<B> {
    /// 如果该 Extent 归属某个 segment 的话，则记录其 ID
    pub fn segment_id(&self) -> u64 {
        let tmp: [u8; 8] = self.buffer.as_ref()[..8].try_into().unwrap();
        u64::from_be_bytes(tmp)
    }
    /// Extent 链表的双向指针
    pub fn flst_node(&self) -> ListNode {
        ListNode::new(&self.buffer.as_ref()[8..20])
    }
    /// XDES_FREE	该 extent 在全局 free 链表上
    /// XDES_FREE_FRAG	该 extent 在全局 free frag 链表上
    /// XDES_FULL_FRAG	该 extent 在全局 full frag 链表上
    /// XDES_FSEG	该 extent 不在任何全局链表上，而是分配给了第一个字段 XDES_ID 记录的值的 segment
    pub fn state(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[20..24].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }

    pub fn bitmap(&self) -> &[u8] {
        &self.buffer.as_ref()[24..40]
    }
}

impl<B: AsRef<[u8]>> fmt::Debug for XDesEntry<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XdesEntry")
            .field("segment_id", &self.segment_id())
            .field("flst_node", &self.flst_node())
            .field("state", &self.state())
            .field("bitmap", &self.bitmap())
            .finish()
    }
}
