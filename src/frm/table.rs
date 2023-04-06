use crate::frm::data_type::DataType;

pub struct Column {
    name: String,
    data_type: DataType,
    len: u16,
    comment: String,
    charset: u8,
    is_null: bool,
}

pub struct Table {

}