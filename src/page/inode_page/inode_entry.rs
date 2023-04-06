use crate::page::ListBaseNode;
use std::fmt;

pub struct InodeEntry<B> {
    buffer: B,
}

impl<B: AsRef<[u8]>> InodeEntry<B> {
    pub fn new(buffer: B) -> InodeEntry<B> {
        assert_eq!(
            buffer.as_ref().len(),
            192,
            "InodeEntry len {}!= 192",
            buffer.as_ref().len()
        );
        Self { buffer }
    }
}

impl<B: AsRef<[u8]>> InodeEntry<B> {
    pub fn segment_id(&self) -> u64 {
        let tmp: [u8; 8] = self.buffer.as_ref()[..8].try_into().unwrap();
        u64::from_be_bytes(tmp)
    }
    pub fn not_full_num(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[8..12].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    pub fn free(&self) -> ListBaseNode {
        ListBaseNode::new(&self.buffer.as_ref()[12..28])
    }
    pub fn not_full(&self) -> ListBaseNode {
        ListBaseNode::new(&self.buffer.as_ref()[28..44])
    }
    pub fn full(&self) -> ListBaseNode {
        ListBaseNode::new(&self.buffer.as_ref()[44..60])
    }

    pub fn magic_number(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[60..64].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    pub fn fragment_array(&self) -> [u32; 32] {
        let mut array = [0 as u32; 32];
        for index in 0..32 {
            let start = 64 + index * 4;
            let end = start + 4;
            let tmp: [u8; 4] = self.buffer.as_ref()[start..end].try_into().unwrap();
            array[index] = u32::from_be_bytes(tmp);
        }
        array
    }
}

impl<B: AsRef<[u8]>> fmt::Debug for InodeEntry<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InodeEntry")
            .field("segment_id", &self.segment_id())
            .field("not_full_num", &self.not_full_num())
            .field("free", &self.free())
            .field("not_full", &self.not_full())
            .field("full", &self.full())
            .field("magic_number", &self.magic_number())
            .field("fragment_array", &self.fragment_array())
            .finish()
    }
}
