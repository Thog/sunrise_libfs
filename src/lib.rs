#![no_std]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate log;

extern crate byteorder;

pub mod fat;

#[derive(Debug)]
pub enum FileSystemError {
    NotFound,
    AccessDenied,
    WriteFailed,
    ReadFailed,
    NoPartitionFound,
    PartitionNotFound,
    UnknownPartitionFormat { partition_type: u32 },
    InvalidPartition,
    Custom,
}

pub enum DirectoryEntryType {
    File,
    Directory,
}

pub struct DirectoryEntry {
    pub path: [u8; 0x301],
    pub entry_type: DirectoryEntryType,
    pub file_size: u64,
}

bitflags! {
    pub struct FileModeFlags: u32 {
        const Readable = 0b00000001;
        const Writable = 0b00000010;
        const Appendable = 0b00000100;
    }
}

bitflags! {
    pub struct DirFilterFlags: u32 {
        const Directory = 0b00000001;
        const File = 0b00000010;
    }
}

type Result<T> = core::result::Result<T, FileSystemError>;

pub trait FileOperations: Sized {
    fn read(&mut self, offset: u64, buf: &mut [u8]) -> Result<u64>;
    fn write_all(&mut self, offset: u64, buf: &[u8]) -> Result<()>;

    fn flush(&mut self) -> Result<()>;
    fn set_len(&mut self, size: u64) -> Result<()>;
    fn get_len(&mut self) -> Result<u64>;
}

pub trait DirectoryOperations: Sized {
    fn read(&mut self, buf: &mut [DirectoryEntry]) -> Result<u64>;
    fn entry_count(&self) -> Result<u64>;
}

pub trait FileSystemOperations: Sized {
    fn create_file(name: &str, mode: FileModeFlags, size: u64) -> Result<()>;
    fn delete_file(name: &str) -> Result<()>;
    fn open_file<T>(name: &str, mode: FileModeFlags) -> Result<T>
    where
        T: FileOperations;

    fn open_directory<T>(name: &str, filter: DirFilterFlags) -> Result<T>
    where
        T: DirectoryOperations;
    fn delete_directory(name: &str) -> Result<()>;
}