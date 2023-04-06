use std::io;

#[derive(Debug)]
pub enum DataValue {
    // 1 字节
    Tinyint(i8),
    // 2字节
    Smallint(i16),
    // 4字节
    Int(i32),
    // 4字节
    Float(f32),
    // 8字节
    Double(f64),
    // 8字节
    Bigint(i64),
    // 3字节
    Mediumint(i32),
    // 4字节
    Timestamp(u32),
    // 1字节
    Year(u8),
    // 3 字节
    Date(u32),
    // 可变
    Varchar(String),

    // 8字节
    Datetime(u64),
    //3字节
    Time(u32),
    Decimal(String),
    // 可变
    Tinytext(String),
    Mediumtext(String),
    Longtext(String),
    Text(String),
    Char(String),
    // 隐藏主键 6字节
    DbRowId(u64),
    // 隐藏事务id 6字节
    DbTrxId(u64),
    //隐藏回滚指针 7字节
    DbRollPtr(u64),
}

impl TryInto<i8> for DataValue {
    type Error = io::Error;

    fn try_into(self) -> Result<i8, Self::Error> {
        if let DataValue::Tinyint(val) = self {
            Ok(val)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("not i8:{:?}", self),
            ))
        }
    }
}
