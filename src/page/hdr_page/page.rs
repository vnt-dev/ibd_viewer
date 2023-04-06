use bytes::Bytes;
use std::fmt;
use std::fmt::{Display, Formatter};

use crate::file_header::FileHeader;
use crate::file_trailer::FileTrailer;
use crate::page::base_page::InternalPage;
use crate::page::hdr_page::header::HdrFspHeader;
use crate::page::hdr_page::x_des_entry::XDesEntry;

#[derive(Debug)]
pub struct FspHdrPage {
    pub fsp_header: HdrFspHeader<Bytes>,
    pub entry_list: Vec<XDesEntry<Bytes>>,
}

impl Display for FspHdrPage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl InternalPage for FspHdrPage {
    fn new(buf: Bytes, _: &FileHeader<Bytes>) -> FspHdrPage {
        let fsp_header = HdrFspHeader::new(buf.slice(..112));
        let mut entry_list = Vec::new();
        let len = fsp_header.free().len
            + fsp_header.free_frag().len
            + fsp_header.full_frag().len
            + fsp_header.seg_inodes_free().len
            + fsp_header.seg_inodes_full().len;
        // let len = 256+2;
        for offset in 0..len as usize {
            let start = 112 + offset * 40;
            let end = start + 40;
            entry_list.push(XDesEntry::new(buf.slice(start..end)))
        }
        // let empty_start = 112 + len as usize * 40;
        // let empty_space = buf.slice(empty_start..);
        Self {
            fsp_header,
            entry_list,
        }
    }
}

// impl<'a> fmt::Debug for FspHdrPage<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("HdrPage")
//             .field("fil_header", &self.fil_header)
//             .field("\nfsp_header", &self.fsp_header)
//             .field("\nentry_list", &self.entry_list)
//             .field("\nempty_space", &self.empty_space)
//             .field("\nfil_trailer", &self.fil_trailer)
//             .finish()
//     }
// }
