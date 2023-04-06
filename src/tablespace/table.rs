use crate::page::base_page::BasePage;
use crate::page::index_page::page::IndexPage;
use crate::page::index_page::records::Row;
use crate::page::sdi_blob_page::SdiBlobPage;
use crate::page::sdi_page::SdiPage;
use crate::tablespace::data_type::DataType;
use crate::tablespace::TableSpace;
use bytes::Bytes;
use serde_json::Value;
use std::collections::HashMap;
use std::io;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub is_nullable: bool,
    pub is_hidden: bool,
    pub ordinal_position: u16,
}

impl Column {
    pub fn new(
        name: String,
        data_type: DataType,
        is_nullable: bool,
        is_hidden: bool,
        ordinal_position: u16,
    ) -> Self {
        Self {
            name,
            data_type,
            is_nullable,
            is_hidden,
            ordinal_position,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Index {
    pub index_id: u64,
    pub root_page_num: u32,
    pub name: String,
    pub is_primary: bool,
    // 索引页列,顺序为物理存储顺序
    pub indexes: Vec<Column>,
    // 叶子页值，顺序为物理存储顺序
    pub elements: Vec<Column>,
}

impl Index {
    pub fn new(
        index_id: u64,
        root_page_num: u32,
        name: String,
        is_primary: bool,
        indexes: Vec<Column>,
        elements: Vec<Column>,
    ) -> Self {
        // let index_
        Self {
            index_id,
            root_page_num,
            name,
            is_primary,
            indexes,
            elements,
        }
    }
    pub fn elements_size(&self) -> usize {
        self.elements.len()
    }
    pub fn index_size(&self) -> usize {
        self.indexes.len()
    }
}

pub struct TableInfo {
    pub name: String,
    pub indexes: HashMap<u64, Index>,
}

pub fn sdi_index(root_page_num: u32) -> Index {
    let c1 = Column::new(String::from("sdi_type"), DataType::Int, false, false, 1);
    let c2 = Column::new(String::from("sdi_id"), DataType::Bigint, false, false, 2);
    let c3 = Column::new(
        String::from("DB_TRX_ID"),
        DataType::DbTrxId,
        false,
        false,
        3,
    );
    let c4 = Column::new(
        String::from("DB_ROLL_PTR"),
        DataType::DbRollPtr,
        false,
        false,
        4,
    );
    let c5 = Column::new(
        String::from("sdi_uncomp_len"),
        DataType::Int,
        false,
        false,
        5,
    );
    let c6 = Column::new(String::from("sdi_comp_len"), DataType::Int, false, false, 6);
    let c7 = Column::new(
        String::from("sdi_value"),
        DataType::Varchar,
        false,
        false,
        7,
    );
    let c_page_num = Column::new(
        String::from("child_page_num"),
        DataType::Int,
        false,
        false,
        8,
    );
    let indexes = vec![c1.clone(), c2.clone(), c_page_num];
    let sdi_elements = vec![c1, c2, c3, c4, c5, c6, c7];
    Index::new(
        18446744073709551615,
        root_page_num,
        "sdi_index".to_string(),
        true,
        indexes,
        sdi_elements,
    )
}

pub fn read_table_info(idb_file_path: &str, page_num: u32) -> io::Result<TableInfo> {
    // 解析sdi数据
    let mut child_page_num = page_num;
    let mut index_page = 'a: loop {
        let buf = TableSpace::read_page_(idb_file_path, child_page_num)?;
        let index_page = BasePage::<SdiPage>::new(buf);
        //找到最左边的叶子
        while index_page.index_header.level() != 0 {
            for row in &index_page.user_records {
                match row {
                    Row::New(row) => {
                        if row.header().delete_mask() == 1 {
                            continue;
                        }
                        let (data, _) = row.col(2);
                        child_page_num = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);

                        continue 'a;
                    }
                }
            }
        }
        break index_page;
    };
    loop {
        for row in &index_page.user_records {
            match row {
                Row::New(row) => {
                    if row.header().delete_mask() == 1 {
                        continue;
                    }
                    let (data, _) = row.col(0);
                    let tmp: [u8; 4] = data.try_into().unwrap();
                    let sdi_type = u32::from_be_bytes(tmp);
                    if sdi_type == 1 {
                        let (str_bytes, overflow_ptr) = row.col(6);
                        let value_bytes = if let Some(ptr) = overflow_ptr {
                            // 读取sdi溢出页
                            let mut bytes = Vec::with_capacity(ptr.len() as usize);
                            bytes.extend_from_slice(str_bytes);

                            let mut next_page_num = ptr.page_num();

                            while next_page_num != u32::MAX {
                                let buf = TableSpace::read_page_(idb_file_path, next_page_num)?;
                                let sdi_blob_page = BasePage::<SdiBlobPage>::new(buf);
                                bytes.extend_from_slice(&sdi_blob_page.data);
                                next_page_num = sdi_blob_page.next_page_num;
                            }
                            bytes
                        } else {
                            str_bytes.to_vec()
                        };
                        let mut decoder = flate2::read::ZlibDecoder::new(value_bytes.as_slice());
                        let mut out = String::new();
                        decoder.read_to_string(&mut out).unwrap();
                        let mut value: Value = serde_json::from_str(&out).unwrap();
                        let table_val = value.get("dd_object").unwrap();
                        let table_name = table_val.get("name").unwrap().as_str().unwrap();
                        let columns = table_val.get("columns").unwrap();
                        let mut map = HashMap::new();
                        let mut column_list = Vec::new();
                        let mut map_col = HashMap::new();
                        for val in columns.as_array().unwrap() {
                            let name = val.get("name").unwrap().as_str().unwrap().to_string();

                            let char_length =
                                val.get("char_length").unwrap().as_u64().unwrap() as usize;
                            let is_nullable = val.get("is_nullable").unwrap().as_bool().unwrap();
                            let is_hidden = val.get("hidden").unwrap().as_u64().unwrap() == 2;
                            let data_type = if is_hidden {
                                match name.as_str() {
                                    "DB_TRX_ID" => DataType::DbTrxId,
                                    "DB_ROW_ID" => DataType::DbRowId,
                                    "DB_ROLL_PTR" => DataType::DbRollPtr,
                                    _ => {
                                        panic!()
                                    }
                                }
                            } else {
                                let data_type = val.get("type").unwrap().as_u64().unwrap() as u8;
                                DataType::new(data_type, char_length)
                            };
                            let ordinal_position =
                                val.get("ordinal_position").unwrap().as_u64().unwrap() as u16;
                            let col = Column::new(
                                name,
                                data_type,
                                is_nullable,
                                is_hidden,
                                ordinal_position,
                            );
                            map_col.insert(ordinal_position - 1, col.clone());
                            column_list.push(col);
                        }
                        let indexes_v = table_val.get("indexes").unwrap().as_array().unwrap();
                        for val in indexes_v {
                            let name = val.get("name").unwrap().as_str().unwrap().to_string();
                            let mut index_id = u64::MAX;
                            let mut root_page_num = u32::MAX;
                            let mut se_private_data = val
                                .get("se_private_data")
                                .unwrap()
                                .as_str()
                                .unwrap()
                                .split(";");
                            for item in se_private_data {
                                if item.is_empty() {
                                    continue;
                                }
                                let mut s = item.split("=");
                                let n = s.next().unwrap();
                                let v = s.next().unwrap();
                                if n == "id" {
                                    index_id = v.parse::<u64>().unwrap();
                                } else if n == "root" {
                                    root_page_num = v.parse::<u32>().unwrap();
                                }
                            }
                            let is_primary = val.get("type").unwrap().as_u64().unwrap() == 1;
                            let elements_v = val.get("elements").unwrap().as_array().unwrap();
                            let mut indexes = Vec::new();
                            let mut elements = Vec::new();
                            for val in elements_v {
                                let len = val.get("length").unwrap().as_u64().unwrap() as u32;
                                let column_opx =
                                    val.get("column_opx").unwrap().as_u64().unwrap() as u16;
                                let col = map_col.get(&column_opx).unwrap();
                                if len < u32::MAX {
                                    indexes.push(col.clone());
                                }
                                elements.push(col.clone());
                            }
                            let index = Index::new(
                                index_id,
                                root_page_num,
                                name,
                                is_primary,
                                indexes,
                                elements,
                            );
                            map.insert(index_id, index);
                        }
                        // println!("{:?}", table_name);
                        // println!("{:?}", map);
                        return Ok(TableInfo {
                            name: table_name.to_string(),
                            indexes: map,
                        });
                    }
                }
            }
        }
        let next = index_page.fil_header.next();
        if next == u32::MAX {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "not fount sdi_type = 1",
            ));
        }
        let buf = TableSpace::read_page_(idb_file_path, next)?;
        index_page = BasePage::<SdiPage>::new(buf);
    }
}
