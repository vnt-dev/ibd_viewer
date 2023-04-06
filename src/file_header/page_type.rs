#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u16)]
pub enum PageType {
    /// 最新分配，还没使用
    FilPageAllocated = 0,
    FilPageTypeUnused = 1,
    /// Undo Log页
    FilPageUndoLog = 2,
    /// 段信息的节点
    FilPageInode = 3,
    /// Insert Buffer空闲列表
    FilPageIbuufFreList = 4,
    /// Insert Buffer位图
    FilPageIbufBitmap = 5,
    /// 系统页
    FilPageTypeSys = 6,
    /// 事务系统数据
    FilPageTypeTrxSys = 7,
    /// File Space Header
    FilPageTypeFspHdr = 8,
    /// 扩展描述页
    FilPageTypeXdes = 9,
    /// BLOB页
    FilPageTypeBlob = 10,
    FilPageTypeZblob = 11,
    FilPageTypeZblob2 = 12,
    FilPageTypeUnknown = 13,
    FilPageCompressed = 14,
    FilPageEncrypted = 15,
    FilPageCompressedAndEncrypted = 16,
    FilPageEncryptedRtree = 17,
    FilPageSdiBlob = 18,
    FilPageSdiZblob = 19,
    FilPageTypeLegacyDblwr = 20,
    FilPageTypeRsegArray = 21,
    FilPageTypeLobIndex = 22,
    FilPageTypeLobData = 23,
    FilPageTypeLobFirst = 24,
    FilPageTypeZlobFirst = 25,
    FilPageTypeZlobData = 26,
    FilPageTypeZlobIndex = 27,
    FilPageTypeZlobFrag = 28,
    FilPageTypeZlobFragEntry = 29,
    FilPageSdi = 17853,
    FilPageRtree = 17854,
    /// B+树的节点
    FilPageIndex = 17855,
}

impl From<u16> for PageType {
    fn from(value: u16) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
