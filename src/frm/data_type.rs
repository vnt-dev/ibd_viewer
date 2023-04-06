#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum DataType {
    MysqlTypeDecimal,
    MysqlTypeTiny,
    MysqlTypeShort,
    MysqlTypeLong,
    MysqlTypeFloat,
    MysqlTypeDouble,
    MysqlTypeNull,
    MysqlTypeTimestamp,
    MysqlTypeLongLong,
    MysqlTypeInt24,
    MysqlTypeDate,
    MysqlTypeTime,
    MysqlTypeDatetime,
    MysqlTypeYear,
    /**< Internal to MySQL. Not used in protocol */
    MysqlTypeNewDate,
    MysqlTypeVarchar,
    MysqlTypeBit,
    MysqlTypeTimestamp2,
    /**< Internal to MySQL. Not used in protocol */
    MysqlTypeDatetime2,
    /**< Internal to MySQL. Not used in protocol */
    MysqlTypeTime2,
    /**< Used for replication only */
    MysqlTypeTypedArray,
    MysqlTypeInvalid = 243,
    /**< Currently just a placeholder */
    MysqlTypeBool = 244,
    MysqlTypeJson = 245,
    MysqlTypeNewDecimal = 246,
    MysqlTypeEnum = 247,
    MysqlTypeSet = 248,
    MysqlTypeTinyBlob = 249,
    MysqlTypeMediumBlob = 250,
    MysqlTypeLongBlob = 251,
    MysqlTypeBlob = 252,
    MysqlTypeVarString = 253,
    MysqlTypeString = 254,
    MysqlTypeGeometry = 255,
}

impl From<u8> for DataType {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}