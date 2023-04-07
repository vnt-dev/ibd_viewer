use crate::file_header::FileHeader;
use crate::file_trailer::FileTrailer;
use crate::page::base_page::InternalIndexPage;
use crate::page::index_page::header::{FSegHeader, IndexHeader};
use crate::page::index_page::records::{new, Row};
use crate::tablespace::data_type::DataType;
use crate::tablespace::table::{Index, TableInfo};
use bytes::{Buf, Bytes};
use std::fmt;
use std::fmt::{Display, Formatter};
use console::style;

#[derive(Debug)]
pub struct IndexPage {
    pub index: Index,
    pub index_header: IndexHeader<Bytes>,
    pub f_seg_header: FSegHeader<Bytes>,
    pub infimum: Row<Bytes>,
    pub supremum: Row<Bytes>,
    pub user_records: Vec<Row<Bytes>>,
    pub free_space: Bytes,
    pub page_directory: Vec<u16>,
}

impl Display for IndexPage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index_header)?;
        write!(f, "{}", self.f_seg_header)?;
        writeln!(f, "{}",style("row:").green())?;
        writeln!(f, "     var&null / not_used / delete_mask / min_rec_mask / n_owned / heap_no / rec_type / next / value")?;
        writeln!(f, " infimum : {}", self.infimum)?;
        writeln!(f, " supremum: {}", self.supremum)?;
        writeln!(f, "{}",style(" user_records:").green())?;

        write!(f, "var&null / not_used / delete_mask / min_rec_mask / n_owned / heap_no / rec_type / next /")?;
        if self.index_header.level() == 0 {
            for col in &self.index.elements {
                write!(f, " {} /", style(&col.name).yellow())?;
            }
        } else {
            for col in &self.index.indexes {
                write!(f, " {} /", style(&col.name).yellow())?;
            }
        }
        writeln!(f, "")?;
        for row in &self.user_records {
            writeln!(f, "  {}", row)?;
        }
        writeln!(f, "page_directory: {:?}", self.page_directory)?;
        Ok(())
    }
}

impl IndexPage {
    pub fn new0(buf: Bytes, index: Index) -> IndexPage {
        let index_header = IndexHeader::new(buf.slice(..36));
        let f_seg_header = FSegHeader::new(buf.slice(36..56));
        let infimum = new::row::Row::new(
            vec![(DataType::Char(8), 0, 8, false, false)],
            5,
            buf.slice(56..56 + 13),
        );
        let supremum = new::row::Row::new(
            vec![(DataType::Char(8), 0, 8, false, false)],
            5,
            buf.slice(56 + 13..56 + 26),
        );
        let num = index_header.heap_num() as usize - 2;
        let mut user_records = Vec::with_capacity(num);
        let mut next = 56 + 5 + infimum.header().next_record();
        let infimum = Row::New(infimum);
        let supremum = Row::New(supremum);
        let columns = if index_header.level() != 0 {
            &index.indexes
        } else {
            &index.elements
        };
        let mut end = 56 + 26;
        for index in 0..num {
            let (start, col_info) = new::row::Row::parse_row_prefix(columns, next as usize, &buf);
            let mut record_len: usize =
                col_info.iter().map(|(_, _, len, _, _)| *len as usize).sum();
            end = (next + record_len as i16) as usize;
            let row = new::row::Row::new(col_info, next as usize - start, buf.slice(start..end));

            next += row.header().next_record();

            user_records.push(Row::New(row));
            if next == 56 + 13 + 5 {
                assert_eq!(index, num - 1, "not last {},{}", next, index);
            }
        }
        let slots = index_header.slots() as usize;
        let empty_end = buf.len() - slots * 2;
        let free_space = buf.slice(end..empty_end);
        let buf_len = buf.len();
        let mut page_directory = Vec::new();
        for slot in 0..slots {
            let i1 = buf_len - 2 - slot * 2;
            let i2 = buf_len - slot * 2;
            let tmp = buf.slice(i1..i2).get_u16();
            page_directory.push(tmp);
        }
        Self {
            index,
            index_header,
            f_seg_header,
            infimum,
            supremum,
            user_records,
            free_space,
            page_directory,
        }
    }
}

impl InternalIndexPage for IndexPage {
    fn new(buf: Bytes, _: &FileHeader<Bytes>, table_info: &TableInfo) -> IndexPage {
        let index_header = IndexHeader::new(buf.slice(..36));
        let index = table_info.indexes.get(&index_header.index_id()).unwrap();
        IndexPage::new0(buf, index.clone())
    }
}

// impl<'a> fmt::Debug for IndexPage<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("IndexPage")
//             .field("fil_header", &self.fil_header)
//             .field("\nindex_header", &self.index_header)
//             .field("\nf_seg_header", &self.f_seg_header)
//             .field("\ninfimum", &self.infimum)
//             .field("\nsupremum", &self.supremum)
//             .field("\nuser_records", &self.user_records)
//             .field("\nfree_space", &self.free_space)
//             .field("\npage_directory", &self.page_directory)
//             .field("\nfil_trailer", &self.fil_trailer)
//             .finish()
//     }
// }
