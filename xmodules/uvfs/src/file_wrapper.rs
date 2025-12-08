/// 文件包装器，用于统一文件抽象

use axerrno::AxResult;

pub struct FileWrapper {
    inner: axfs::api::File,
    offset: usize,
}

impl FileWrapper {
    pub fn new(file: axfs::api::File) -> Self {
        Self { inner: file, offset: 0 }
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
}
