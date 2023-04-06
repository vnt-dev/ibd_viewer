use std::fmt::Formatter;
use std::{fmt, io};

pub mod new;

pub enum Row<B> {
    New(new::row::Row<B>),
}

impl<B: AsRef<[u8]>> fmt::Display for Row<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Row::New(row) => {
                writeln!(f, "{}", row)?;
                write!(f, " raw data: ")?;
                for x in row.buf() {
                    write!(f, "{:02x} ", x)?;
                }
            }
        }
        Ok(())
    }
}

impl<B: AsRef<[u8]>> fmt::Debug for Row<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Row::New(row) => row.fmt(f),
        }
    }
}
