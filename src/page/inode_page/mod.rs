use crate::file_header::FileHeader;
use crate::file_trailer::FileTrailer;
use crate::page::base_page::InternalPage;
use crate::page::inode_page::inode_entry::InodeEntry;
use crate::page::ListNode;
use bytes::Bytes;
use std::fmt;
use std::fmt::{Display, Formatter};

pub mod inode_entry;

#[derive(Debug)]
pub struct InodePage {
    pub list_node: ListNode,
    pub inode_list: Vec<InodeEntry<Bytes>>,
    pub empty_space: Bytes,
}

impl Display for InodePage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl InternalPage for InodePage {
    fn new(buf: Bytes, _: &FileHeader<Bytes>) -> InodePage {
        let list_node = ListNode::new(&buf[..12]);
        let mut inode_list = Vec::with_capacity(85);
        let mut end = 0;
        for index in 0..85 {
            let start = 12 + 192 * index;
            end = start + 192;
            let entry = InodeEntry::new(buf.slice(start..end));
            inode_list.push(entry);
        }
        let empty_space = buf.slice(end..);
        Self {
            list_node,
            inode_list,
            empty_space,
        }
    }
}
//
// impl<'a> fmt::Debug for InodePage<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("InodePage")
//             .field("fil_header", &self.fil_header)
//             .field("\nlist_node", &self.list_node)
//             .field("\ninode_list", &self.inode_list)
//             .field("\nempty_space", &self.empty_space)
//             .field("\nfil_trailer", &self.fil_trailer)
//             .finish()
//     }
// }
