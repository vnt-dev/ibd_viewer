use std::fmt;

pub struct FileTrailer<B> {
    buffer: B,
}

impl<B: AsRef<[u8]>> FileTrailer<B> {
    pub fn new(buffer: B) -> FileTrailer<B> {
        assert_eq!(
            buffer.as_ref().len(),
            8,
            "RecordHeader len {}!= 8",
            buffer.as_ref().len()
        );
        Self { buffer }
    }
}

impl<B: AsRef<[u8]>> FileTrailer<B> {
    pub fn check_sum(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[..4].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
    pub fn lsn(&self) -> u32 {
        let tmp: [u8; 4] = self.buffer.as_ref()[4..8].try_into().unwrap();
        u32::from_be_bytes(tmp)
    }
}
impl<B: AsRef<[u8]>> fmt::Display for FileTrailer<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "FileTrailer")?;
        writeln!(
            f,
            " check_sum:{} ({})",
            self.check_sum(),
            hex::encode(&self.buffer.as_ref()[..4])
        )?;
        writeln!(
            f,
            " lsn:{} ({})",
            self.lsn(),
            hex::encode(&self.buffer.as_ref()[4..8])
        )
    }
}
impl<B: AsRef<[u8]>> fmt::Debug for FileTrailer<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileTrailer")
            .field("check_sum", &self.check_sum())
            .field("lsn", &self.lsn())
            .finish()
    }
}
