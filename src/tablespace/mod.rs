use crate::file_header::page_type::PageType;
use crate::page;
use crate::page::base_page::BasePage;
use crate::page::hdr_page::page::FspHdrPage;
use crate::page::PageEnums;
use crate::tablespace::table::{read_table_info, Index, TableInfo};
use bytes::Bytes;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};

pub mod data_type;
pub mod data_value;
pub mod table;

pub struct TableSpace {
    pub table_info: TableInfo,
    pub idb_file_path: String,
}

impl TableSpace {
    pub fn new(idb_file_path: String) -> io::Result<Self> {
        // todo 从fsp页中读取sdi页号
        let table_info = read_table_info(&idb_file_path, 3)?;
        Ok(Self {
            table_info,
            idb_file_path,
        })
    }
}

impl TableSpace {
    pub fn fsp_page(&self) -> io::Result<BasePage<FspHdrPage>> {
        let buf = self.read_page(0)?;
        Ok(BasePage::new(buf))
    }
    pub fn page(&self, page_num: u32) -> io::Result<PageEnums> {
        let buf = self.read_page(page_num)?;
        Ok(page::page(buf, &self.table_info))
    }
    pub fn index_roots(&self) -> Vec<(String, u32)> {
        let mut v = Vec::new();
        for (_, index) in &self.table_info.indexes {
            v.push((index.name.clone(), index.root_page_num));
        }
        v
    }
}

impl TableSpace {
    pub fn index(&self, index_id: u64) -> Option<&Index> {
        self.table_info.indexes.get(&index_id)
    }
    pub fn read_page(&self, page_num: u32) -> io::Result<Bytes> {
        TableSpace::read_page_(&self.idb_file_path, page_num)
    }
    pub fn read_page_(idb_file_path: &str, page_num: u32) -> io::Result<Bytes> {
        let mut file = File::open(idb_file_path)?;
        let _ = file.seek(SeekFrom::Start(page_num as u64 * 16 * 1024))?;
        let mut buf = vec![0; 16 * 1024];
        file.read_exact(&mut buf)?;
        Ok(Bytes::from(buf))
    }
}
