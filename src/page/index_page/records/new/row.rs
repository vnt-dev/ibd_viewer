use crate::page::index_page::records::new::header::{RecordHeader, NEW_HEAD_LEN};
use crate::page::{ListNode, OverflowPointer};
use crate::tablespace::data_type::DataType;
use crate::tablespace::table::{Column, Index};
use std::fmt;
use std::fmt::Formatter;
use std::io::Read;

pub struct Row<B> {
    col_info: Vec<(DataType, u16, u16, bool, bool)>,
    rec_offset: usize,
    buffer: B,
}

impl<B: AsRef<[u8]>> Row<B> {
    pub fn new(
        col_info: Vec<(DataType, u16, u16, bool, bool)>,
        rec_offset: usize,
        buffer: B,
    ) -> Row<B> {
        Self {
            col_info,
            rec_offset,
            buffer,
        }
    }
    pub fn parse_row_prefix(
        columns: &Vec<Column>,
        mut rec_offset: usize,
        buffer: B,
    ) -> (usize, Vec<(DataType, u16, u16, bool, bool)>) {
        //计算列信息，(类型,偏移量,长度,是否溢出,是否为空)  注意长度为0，值不为空的情况(空字符串)
        let mut col_info: Vec<(DataType, u16, u16, bool, bool)> = Vec::with_capacity(columns.len());
        let buf = buffer.as_ref();
        rec_offset -= NEW_HEAD_LEN;
        let mut null_num = 0;
        //处理空值列表 (data_type, is_nullable)
        for column in columns {
            let data_type = column.data_type;
            let is_nullable = column.is_nullable;
            if is_nullable {
                let val = buf[rec_offset - null_num / 8 - 1];
                if (val >> (null_num % 8)) & 0b1 == 0b1 {
                    col_info.push((data_type, 0, 0, false, true));
                } else {
                    col_info.push((data_type, 0, data_type.len() as u16, false, false));
                }
                null_num += 1;
            } else {
                col_info.push((data_type, 0, data_type.len() as u16, false, false));
            }
        }
        rec_offset -= null_num / 8;
        if null_num % 8 != 0 {
            rec_offset -= 1;
        }
        // 处理变长列表
        for (data_type, _, len, is_overflow, is_null) in col_info.iter_mut() {
            match *data_type {
                DataType::Varchar
                | DataType::Tinytext
                | DataType::Mediumtext
                | DataType::Longtext
                | DataType::Text => {
                    if !*is_null {
                        rec_offset -= 1;
                        let val = buf[rec_offset] as u16;
                        if val & 0x80 == 0 {
                            *len = val;
                        } else {
                            rec_offset -= 1;
                            *len = buf[rec_offset] as u16 + ((val & 0x3f) << 8);
                            // 两个字节
                            if val & 0x40 == 0x40 {
                                *is_overflow = true;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        let mut last_offset = 0;
        for (_, offset, len, _, _) in col_info.iter_mut() {
            *offset = last_offset;
            last_offset += *len;
        }
        (rec_offset, col_info)
    }
}

impl<B: AsRef<[u8]>> Row<B> {
    // pub fn var_list(&self)->Vec<>
    pub fn header(&self) -> RecordHeader<&[u8]> {
        let start = self.rec_offset - NEW_HEAD_LEN;
        let end = self.rec_offset;
        RecordHeader::new(&self.buffer.as_ref()[start..end])
    }
    pub fn data(&self) -> &[u8] {
        &self.buffer.as_ref()[self.rec_offset..]
    }
    pub fn buf(&self) -> &[u8] {
        self.buffer.as_ref()
    }
    pub fn col(&self, index: usize) -> (&[u8], Option<OverflowPointer<&[u8]>>) {
        let (_, offset, len, is_overflow, _) = self.col_info[index];
        let data = &self.data()[offset as usize..offset as usize + len as usize];
        if is_overflow {
            let start = (len & 0xFF00) as usize;
            (&data[..start], Some(OverflowPointer::new(&data[start..])))
        } else {
            (data, None)
        }
    }
    pub fn col_type(&self, index: usize) -> (DataType, &[u8], Option<OverflowPointer<&[u8]>>) {
        let (data_type, offset, len, is_overflow, _) = self.col_info[index];
        let data = &self.data()[offset as usize..offset as usize + len as usize];
        if is_overflow {
            let start = (len & 0xFF00) as usize;
            (
                data_type,
                &data[..start],
                Some(OverflowPointer::new(&data[start..])),
            )
        } else {
            (data_type, data, None)
        }
    }

    pub fn col_info(&self) -> &Vec<(DataType, u16, u16, bool, bool)> {
        &self.col_info
    }
    pub fn size(&self) -> usize {
        self.buffer.as_ref().len()
    }
}

impl<B: AsRef<[u8]>> fmt::Display for Row<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ", &self.buf()[..self.rec_offset - 5])?;
        write!(f, "{} ", self.header())?;
        for index in 0..self.col_info().len() {
            let (data_type, col, ptr) = self.col_type(index);
            if data_type.is_str() {
                write!(f, " [")?;
                for x in col {
                    write!(f, " {:02x}", x)?;
                }
                write!(f, "]")?;
            } else {
                write!(f, "{} ", hex::encode(col))?;
            }

            if let Some(ptr) = ptr {
                write!(f, "(overflow {}", ptr)?;
                write!(f, ",raw:")?;
                for x in ptr.buf {
                    write!(f, "{:02x} ", x)?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

impl<B: AsRef<[u8]>> fmt::Debug for Row<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Row")
            .field("col_info", &self.col_info())
            .field("header", &self.header())
            .field("data", &self.data())
            .finish()
    }
}
