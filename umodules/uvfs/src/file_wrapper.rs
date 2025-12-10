/// 文件包装器，用于统一文件抽象

use axerrno::{AxResult, AxError};
use axfs::api::OpenOptions;

pub struct FileWrapper {
    pub inner: axfs::api::File,
    pub offset: usize,
    pub flags: u32,
}

impl FileWrapper {
    pub fn new(file: axfs::api::File) -> Self {
        Self { 
            inner: file, 
            offset: 0,
            flags: 0,
        }
    }

    pub fn with_flags(file: axfs::api::File, flags: u32) -> Self {
        Self {
            inner: file,
            offset: 0,
            flags,
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> AxResult<usize> {
        let n = self.inner.read(buf)?;
        self.offset += n;
        Ok(n)
    }

    pub fn write(&mut self, buf: &[u8]) -> AxResult<usize> {
        let n = self.inner.write(buf)?;
        self.offset += n;
        Ok(n)
    }

    /// lseek 实现
    pub fn seek(&mut self, offset: i64, whence: i32) -> AxResult<usize> {
        let new_offset = match whence {
            0 => offset as usize, // SEEK_SET
            1 => (self.offset as i64 + offset) as usize, // SEEK_CUR
            2 => {
                // SEEK_END - 需要文件大小
                let size = self.inner.metadata()?.len() as i64;
                (size + offset) as usize
            }
            _ => return Err(AxError::InvalidInput),
        };
        
        self.offset = new_offset;
        Ok(new_offset)
    }

    /// 获取文件元数据
    pub fn metadata(&self) -> AxResult<axfs::api::FileMetadata> {
        self.inner.metadata()
    }

    /// 截断文件
    pub fn truncate(&mut self, len: usize) -> AxResult {
        self.inner.truncate(len as u64)
    }
}
