use std::io::{Read, Seek, SeekFrom};

use crate::err::fs_err::FileSystemError;

pub trait ReadSeek: Read + Seek {}
impl<R: Read + Seek + Send + Sync> ReadSeek for R {}

/// Provide a generic type => Detect the size of the data structure => read data in memory and unsafe enforce the type
pub fn reader_read_val<T>(reader: &mut dyn ReadSeek) -> Result<(T, usize), FileSystemError> {
    let mut t: T = unsafe { std::mem::zeroed() };
    let t_buf = unsafe {
        std::slice::from_raw_parts_mut(&mut t as *mut _ as *mut u8, std::mem::size_of::<T>())
    };
    reader.read_exact(t_buf)?;
    Ok((t, t_buf.len()))
}

// Same as a normal data reader, but with offset
pub struct ByteDataReader {
    offset: usize,
    data: Vec<u8>,
}

impl ByteDataReader {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            offset: 0,
            data: data,
        }
    }
}

impl Read for ByteDataReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let end = std::cmp::min(self.data.len(), self.offset + buf.len());
        let size = end - self.offset;
        buf.copy_from_slice(&self.data[self.offset..end]);
        self.offset = end;
        Ok(size)
    }
}

impl Seek for ByteDataReader {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, std::io::Error> {
        match pos {
            SeekFrom::Current(pos_val) => {
                let new_offset = self.offset as i64 + pos_val;
                self.offset = new_offset as usize;
            }
            SeekFrom::Start(pos_val) => self.offset = pos_val as usize,
            SeekFrom::End(pos_val) => {
                let new_offset = self.data.len() as i64 + pos_val;
                self.offset = new_offset as usize;
            }
        };

        Ok(self.offset as u64)
    }
}
