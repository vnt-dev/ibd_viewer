use crate::file_header::FileHeader;
use crate::file_trailer::FileTrailer;
use crate::page::base_page::{InternalIndexPage, InternalPage};
use crate::page::index_page::page::IndexPage;
use crate::page::index_page::records::Row;
use crate::tablespace::table::sdi_index;
use bytes::Bytes;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::ops::Deref;

#[derive(Debug)]
pub struct SdiPage {
    pub original: IndexPage,
}

impl Deref for SdiPage {
    type Target = IndexPage;

    fn deref(&self) -> &Self::Target {
        &self.original
    }
}

impl Display for SdiPage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.original)
    }
}

impl InternalPage for SdiPage {
    fn new(buf: Bytes, fil_header: &FileHeader<Bytes>) -> SdiPage {
        let index = sdi_index(fil_header.offset());
        let original = IndexPage::new0(buf, index);
        Self { original }
    }
}
