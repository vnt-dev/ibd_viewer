use bytes::Bytes;
use index_page::header::IndexHeader;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::file_header::page_type::PageType;
use crate::file_header::FileHeader;
use crate::page::base_page::{BasePage, UnKnowPage};
use crate::page::hdr_page::page::FspHdrPage;
use crate::page::index_page::page::IndexPage;
use crate::page::inode_page::InodePage;
use crate::page::sdi_blob_page::SdiBlobPage;
use crate::page::sdi_page::SdiPage;
use crate::tablespace::table::TableInfo;

pub mod base_page;
pub mod hdr_page;
pub mod index_page;
pub mod inode_page;
pub mod lob_first_page;
pub mod sdi_blob_page;
pub mod sdi_page;

/// 在 InnoDB 里链表头叫做 FLST _BASE_NODE，大小为 FLST_BASE_NODE_SIZE（6+6+4字节）。
/// FLST _BASE_NODE 中包含链表的头尾指针，链表中每个节点称为 FLST_NODE，大小为 FLST_NODE_SIZE（6+6字节）
#[derive(Debug, Copy, Clone)]
pub struct ListNode {
    pub prev: FilePointer,
    pub next: FilePointer,
}

impl ListNode {
    pub fn new(buffer: &[u8]) -> Self {
        let prev = FilePointer::new(&buffer[..6]);
        let next = FilePointer::new(&buffer[6..12]);
        Self { prev, next }
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "prev:{},next:{}", self.prev, self.next)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ListBaseNode {
    pub len: u32,
    pub first: FilePointer,
    pub last: FilePointer,
}

impl ListBaseNode {
    pub fn new(buffer: &[u8]) -> Self {
        let tmp: [u8; 4] = buffer[..4].try_into().unwrap();
        let len = u32::from_be_bytes(tmp);
        let first = FilePointer::new(&buffer[4..10]);
        let last = FilePointer::new(&buffer[10..16]);
        Self { len, first, last }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FilePointer {
    /// space 内 page 号
    pub page_num: u32,
    /// page 内 offset
    pub offset: u16,
}

impl FilePointer {
    pub fn new(buffer: &[u8]) -> Self {
        let tmp: [u8; 4] = buffer[..4].try_into().unwrap();
        let page_num = u32::from_be_bytes(tmp);
        let tmp: [u8; 2] = buffer[4..6].try_into().unwrap();
        let offset = u16::from_be_bytes(tmp);
        Self { page_num, offset }
    }
}

impl fmt::Display for FilePointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "page_num:{} offset:{}", self.page_num, self.offset)
    }
}

/// 溢出指针
#[derive(Debug, Copy, Clone)]
pub struct OverflowPointer<B> {
    pub buf: B,
}

impl<B: AsRef<[u8]>> OverflowPointer<B> {
    pub fn new(buf: B) -> OverflowPointer<B> {
        Self { buf }
    }
    pub fn space_id(&self) -> u32 {
        let tmp: [u8; 4] = self.buf.as_ref()[..4].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    pub fn page_num(&self) -> u32 {
        let tmp: [u8; 4] = self.buf.as_ref()[4..8].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    /// 未知含义
    pub fn un_know(&self) -> u32 {
        let tmp: [u8; 4] = self.buf.as_ref()[8..12].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    /// 数据长度
    pub fn len(&self) -> u64 {
        let tmp: [u8; 8] = self.buf.as_ref()[12..].try_into().unwrap();
        u64::from_be_bytes(tmp)
    }
}

impl<B: AsRef<[u8]>> fmt::Display for OverflowPointer<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "space_id:{} page:{} unknow:{} len:{}",
            self.space_id(),
            self.page_num(),
            self.un_know(),
            self.len()
        )
    }
}

pub enum PageEnums {
    Index(BasePage<IndexPage>),
    FspHdr(BasePage<FspHdrPage>),
    XDex(BasePage<FspHdrPage>),
    Inode(BasePage<InodePage>),
    Sdi(BasePage<SdiPage>),
    SdiBlob(BasePage<SdiBlobPage>),
    Undefine(BasePage<UnKnowPage>),
}

impl PageEnums {
    pub fn page_type(&self) -> PageType {
        match self {
            PageEnums::Index(page) => page.fil_header.page_type(),
            PageEnums::FspHdr(page) => page.fil_header.page_type(),
            PageEnums::XDex(page) => page.fil_header.page_type(),
            PageEnums::Inode(page) => page.fil_header.page_type(),
            PageEnums::Sdi(page) => page.fil_header.page_type(),
            PageEnums::SdiBlob(page) => page.fil_header.page_type(),
            PageEnums::Undefine(page) => page.fil_header.page_type(),
        }
    }
}

pub fn page(buf: Bytes, table_info: &TableInfo) -> PageEnums {
    let fil = FileHeader::new(buf.slice(..38));

    match fil.page_type() {
        PageType::FilPageTypeFspHdr => PageEnums::FspHdr(BasePage::new(buf)),
        PageType::FilPageTypeXdes => PageEnums::FspHdr(BasePage::new(buf)),
        PageType::FilPageIndex => PageEnums::Index(BasePage::new_index(buf, table_info)),
        PageType::FilPageInode => PageEnums::Inode(BasePage::new(buf)),
        PageType::FilPageSdi => PageEnums::Sdi(BasePage::new(buf)),
        _ => PageEnums::Undefine(BasePage::new(buf)),
    }
}

impl Display for PageEnums {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PageEnums::Index(page) => {
                write!(f, "{}", page)
            }
            PageEnums::FspHdr(page) => {
                write!(f, "{}", page)
            }
            PageEnums::XDex(page) => {
                write!(f, "{}", page)
            }
            PageEnums::Inode(page) => {
                write!(f, "{}", page)
            }
            PageEnums::Sdi(page) => {
                write!(f, "{}", page)
            }
            PageEnums::SdiBlob(page) => {
                write!(f, "{}", page)
            }
            PageEnums::Undefine(page) => {
                write!(f, "{}", page)
            }
        }
    }
}
