use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};

use crate::{
    error::FileSystemError,
    reader::{reader_read_val, ByteDataReader, ReadSeek},
};

#[cfg(target_pointer_width = "64")]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
struct PFS0FileEntry {
    offset: u64,
    /// usize in a 64-bit target is 0x8 = 8 bytes
    /// because _offset_ is defined to be 0x8 => enforce the Rust architecture
    /// to work with 64-bit target only
    size: usize,
    name_offset: u32,
    padding: u32,
}

type PFS0FileEntryTable = Vec<PFS0FileEntry>;

/// ## PFS0 File Header Format
#[derive(Clone, Debug)]
#[cfg(target_pointer_width = "64")]
#[repr(C)]
struct PFS0FileHeader {
    /// 0x0: The "PFS0" magic (0x30534650 in LE) - (4 bytes)
    pub magic: u32,
    /// 0x4: Number of files within the PFS - (4 bytes)
    pub file_count: u32,
    /// 0x8: Size of the string table - (4 bytes)
    pub string_table_size: u32,
    /// 0xC: Padding all zeros - (4 bytes)
    pub reserved: [u8; 0x4],
}

impl PFS0FileHeader {
    /// The "PFS0" magic (0x30534650 in LE)
    pub const PFS0_MAGIC: u32 = u32::from_le_bytes(*b"PFS0");
}

/// # Partition File System
/// PFS0 (Partition FS) format is used within NCAs, and is a Switch-exclusive format
/// File format resource: https://wiki.oatmealdome.me/PFS0_(File_Format)
pub struct PFS0 {
    size: usize,
    header: PFS0FileHeader,
    reader: Box<dyn ReadSeek>,
    file_entries: PFS0FileEntryTable,
    string_table: Vec<u8>,
}

impl PFS0 {
    pub fn open_file(file_path: &str) -> Result<Self, FileSystemError> {
        let mut size_pt = 0;
        // Read file binary data, return type uint8array
        let file_bytes_data = match std::fs::read(file_path) {
            Ok(bytes) => bytes,
            Err(err) => return Err(FileSystemError::FileOpenedError(err.to_string())),
        };

        // Construct byte data reader to parse the PFS0 header
        let mut reader = ByteDataReader::new(file_bytes_data);

        // The "PFS0" magic (0x30534650 in LE)
        let (header, header_size): (PFS0FileHeader, usize) = reader_read_val(&mut reader)?;
        if header.magic != PFS0FileHeader::PFS0_MAGIC {
            return Err(FileSystemError::InvalidPFS0Magic(header.magic));
        }
        size_pt += header_size;

        // Array of file entry tables
        let mut file_entries: PFS0FileEntryTable = Vec::with_capacity(header.file_count as usize);
        for _ in 0..header.file_count as usize {
            let (curr_header_file_entry, curr_header_file_size): (PFS0FileEntry, usize) =
                reader_read_val(&mut reader)?;
            println!("> File entry: {:?}", curr_header_file_entry);
            file_entries.push(curr_header_file_entry);

            size_pt += curr_header_file_size;
        }

        // Array of null-terminated filename strings, padded to 0x20
        let mut string_table: Vec<u8> = vec![0u8; header.string_table_size as usize];
        reader.read_exact(&mut string_table)?;

        size_pt += string_table.len();

        Ok(PFS0 {
            size: size_pt,
            header,
            file_entries,
            reader: Box::new(reader),
            string_table,
        })
    }

    /// List all file entry names
    pub fn list_files(&self) -> Result<Vec<String>, FileSystemError> {
        let mut file_names: Vec<String> = Vec::with_capacity(self.file_entries.len());

        for entry in self.file_entries.iter() {
            let mut entry_reader = ByteDataReader::new(self.string_table.to_vec());
            // Jump to the _name_offset_ position
            entry_reader.seek(SeekFrom::Start(entry.name_offset as u64))?;

            // Read the bytes of file name (slice data after the _name_offset_)
            let mut name_bytes = vec![0u8; entry.size - entry.name_offset as usize];
            entry_reader.read(&mut name_bytes)?;

            // Convert byte to string and push to array
            file_names.push(String::from_utf8(name_bytes).unwrap());
        }

        Ok(file_names)
    }

    /// Get size of file
    pub fn get_file_entry(&self, idx: usize) -> Result<PFS0FileEntry, FileSystemError> {
        if idx >= self.file_entries.len() {
            return Err(FileSystemError::InvalidInput(
                "invalid file index".to_string(),
            ));
        }
        Ok(self.file_entries[idx])
    }

    pub fn read_file_data(
        &mut self,
        idx: usize,
        offset: usize,
        buf: &mut [u8],
    ) -> Result<usize, FileSystemError> {
        if idx >= self.file_entries.len() {
            return Err(FileSystemError::InvalidInput(
                "invalid file index".to_string(),
            ));
        }

        let file_entry = self.get_file_entry(idx)?;
        if buf.len() + offset > file_entry.size {
            return Err(FileSystemError::from(Error::new(
                ErrorKind::UnexpectedEof,
                "EOF reached",
            )));
        }

        // retrieve metadata size of pfs0 file
        let base_offset = std::mem::size_of::<PFS0FileHeader>()
            + std::mem::size_of::<PFS0FileEntry>() * self.header.file_count as usize
            + self.header.string_table_size as usize;
        // jump to the current file entry byte alignment in the memory
        let base_read_offset = base_offset + file_entry.offset as usize;
        let read_offset = base_read_offset + offset;

        // Jump to the _read_offset_ position
        self.reader.seek(SeekFrom::Start(read_offset as u64))?;
        let data = self.reader.read(buf)?;
        Ok(data)
    }
}
