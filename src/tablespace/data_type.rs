use crate::tablespace::data_type::DataType::{
    Bigint, Bit, Char, Date, Datetime, Decimal, Double, Float, Int, Longtext, Mediumint,
    Mediumtext, Smallint, Text, Time, Timestamp, Tinyint, Tinytext, UnKnow, Varchar, Year,
};
use std::io;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum DataType {
    // 1 字节
    Tinyint = 2,
    // 2字节
    Smallint = 3,
    // 4字节
    Int = 4,
    // 4字节
    Float = 5,
    // 8字节
    Double = 6,
    // 8字节
    Bigint = 9,
    // 3字节
    Mediumint = 10,
    // 4字节
    Timestamp = 13,
    // 1字节
    Year = 14,
    // 3 字节
    Date = 15,
    // 可变
    Varchar = 16,
    Bit = 17,

    // 8字节
    Datetime = 19,
    //3字节
    Time = 20,
    // 1–2 1
    // 3–4 2
    // 5–6 3
    // 7–9 4
    Decimal(usize) = 21,
    // 可变
    Tinytext = 24,
    Mediumtext = 25,
    Longtext = 26,
    Text = 27,
    Char(usize) = 29,
    // 隐藏主键 6字节
    DbRowId,
    // 隐藏事务id 6字节
    DbTrxId,
    //隐藏回滚指针 7字节
    DbRollPtr,
    UnKnow(u8, usize),
}

impl DataType {
    pub fn new(value: u8, len: usize) -> Self {
        match value {
            2 => Tinyint,
            3 => Smallint,
            4 => Int,
            5 => Float,
            6 => Double,
            9 => Bigint,
            10 => Mediumint,
            13 => Timestamp,
            14 => Year,
            15 => Date,
            16 => Varchar,
            17 => Bit,
            19 => Datetime,
            20 => Time,
            21 => Decimal(len),
            24 => Tinytext,
            25 => Mediumtext,
            26 => Longtext,
            27 => Text,
            29 => Char(len),
            _ => UnKnow(value, len),
        }
    }
    pub fn len(&self) -> usize {
        match self {
            Tinyint => 1,
            Smallint => 2,
            Int => 4,
            Float => 4,
            Double => 8,
            Bigint => 8,
            Mediumint => 3,
            Timestamp => 4,
            Year => 1,
            Date => 3,
            Varchar => 0,
            Bit => 1,
            Datetime => 8,
            Time => 3,
            Decimal(len) => *len,
            Tinytext => 0,
            Mediumtext => 0,
            Longtext => 0,
            Text => 0,
            Char(len) => *len,
            UnKnow(_, _) => panic!(),
            DataType::DbRowId => 6,
            DataType::DbTrxId => 6,
            DataType::DbRollPtr => 7,
        }
    }
    pub fn is_var(&self) -> bool {
        match self {
            Varchar | Tinytext | Mediumtext | Longtext | Text => true,
            _ => false,
        }
    }
    pub fn is_str(&self) -> bool {
        match self {
            Varchar | Tinytext | Mediumtext | Longtext | Text | Char(_) => true,
            _ => false,
        }
    }
}
