use crate::file_header::FileHeader;
use crate::file_trailer::FileTrailer;
use crate::page::base_page::InternalPage;
use bytes::{Buf, Bytes};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct SdiBlobPage {
    pub part_len: u32,
    pub next_page_num: u32,
    pub data: Bytes,
}

impl Display for SdiBlobPage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl InternalPage for SdiBlobPage {
    fn new(buf: Bytes, _: &FileHeader<Bytes>) -> SdiBlobPage {
        let part_len = buf.slice(..4).get_u32();
        let next_page_num = buf.slice(4..8).get_u32();
        let data = buf.slice(8..8 + part_len as usize);
        Self {
            part_len,
            next_page_num,
            data,
        }
    }
}
