use crate::file_header::FileHeader;
use crate::file_trailer::FileTrailer;
use crate::tablespace::table::{Index, TableInfo};
use bytes::Bytes;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

pub struct BasePage<P> {
    pub buf: Bytes,
    pub fil_header: FileHeader<Bytes>,
    pub page: P,
    pub fil_trailer: FileTrailer<Bytes>,
}

pub struct UnKnowPage;

impl Display for UnKnowPage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl InternalPage for UnKnowPage {
    fn new(buf: Bytes, fil_header: &FileHeader<Bytes>) -> Self {
        UnKnowPage
    }
}

pub trait InternalPage: Display {
    fn new(buf: Bytes, fil_header: &FileHeader<Bytes>) -> Self;
}

pub trait InternalIndexPage: Display {
    fn new(buf: Bytes, fil_header: &FileHeader<Bytes>, table_info: &TableInfo) -> Self;
}

impl<P> Deref for BasePage<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.page
    }
}

impl<P: Display> Display for BasePage<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.fil_header)?;
        writeln!(f, "{}", self.page)?;
        writeln!(f, "{}", self.fil_trailer)
    }
}

impl<P: Display> BasePage<P> {
    pub fn new0(buf: Bytes, page: P) -> BasePage<P> {
        let len = buf.len();
        let fil_header = FileHeader::new(buf.slice(..38));
        let fil_trailer = FileTrailer::new(buf.slice(len - 8..));
        Self {
            buf,
            fil_header,
            page,
            fil_trailer,
        }
    }
}

impl<P: InternalPage> BasePage<P> {
    pub fn new(buf: Bytes) -> BasePage<P> {
        let len = buf.len();
        let fil_header = FileHeader::new(buf.slice(..38));
        let page = InternalPage::new(buf.slice(38..len - 8), &fil_header);
        let fil_trailer = FileTrailer::new(buf.slice(len - 8..));
        Self {
            buf,
            fil_header,
            page,
            fil_trailer,
        }
    }
}

impl<P: InternalIndexPage> BasePage<P> {
    pub fn new_index(buf: Bytes, table_info: &TableInfo) -> BasePage<P> {
        let len = buf.len();
        let fil_header = FileHeader::new(buf.slice(..38));
        let page = InternalIndexPage::new(buf.slice(38..len - 8), &fil_header, table_info);
        let fil_trailer = FileTrailer::new(buf.slice(len - 8..));
        Self {
            buf,
            fil_header,
            page,
            fil_trailer,
        }
    }
}
