/// VFS操作抽象层
use axerrno::{AxResult, AxError};
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use spin::Mutex;
use crate::FileWrapper;

extern crate unotify;

// 全局文件描述符表
static FILE_TABLE: Mutex<Vec<Option<FileWrapper>>> = Mutex::new(Vec::new());

pub struct VfsOps;

impl VfsOps {
    /// 打开文件，返回文件描述符
    pub fn open(path: &str, flags: u32, mode: u32) -> AxResult<usize> {
        log::debug!("VfsOps::open: {} (flags={}, mode={})", path, flags, mode);
        
        // 调用ArceOS的axfs打开文件
        let file = axfs::api::File::open(path)?;
        let wrapper = FileWrapper::new(file);
        
        // 分配文件描述符
        let mut table = FILE_TABLE.lock();
        let fd = table.len();
        table.push(Some(wrapper));
        
        // 触发文件访问事件
        let watcher = unotify::get_watcher();
        let event = unotify::NotifyEvent::new(
            unotify::EventType::ACCESS,
            path.to_string(),
        );
        watcher.trigger(event);
        
        log::trace!("File opened: {} -> fd={}", path, fd);
        Ok(fd)
    }

    /// 从文件读取
    pub fn read(fd: usize, buf: &mut [u8]) -> AxResult<usize> {
        log::trace!("VfsOps::read: fd={}, len={}", fd, buf.len());
        
        // 直接从文件读取
        let mut table = FILE_TABLE.lock();
        let file_wrapper = table.get_mut(fd)
            .and_then(|opt| opt.as_mut())
            .ok_or(AxError::BadState)?;
        
        let n = file_wrapper.read(buf)?;
        
        log::trace!("Read {} bytes from fd={}", n, fd);
        Ok(n)
    }

    /// 向文件写入，触发通知
    pub fn write(fd: usize, buf: &[u8]) -> AxResult<usize> {
        log::trace!("VfsOps::write: fd={}, len={}", fd, buf.len());
        
        // 直接写入文件
        let mut table = FILE_TABLE.lock();
        let file_wrapper = table.get_mut(fd)
            .and_then(|opt| opt.as_mut())
            .ok_or(AxError::BadState)?;
        
        let n = file_wrapper.write(buf)?;
        drop(table);
        
        // 触发文件修改事件
        let watcher = unotify::get_watcher();
        let event = unotify::NotifyEvent::new(
            unotify::EventType::MODIFY,
            alloc::format!("fd_{}", fd),
        );
        watcher.trigger(event);
        
        log::trace!("Wrote {} bytes to fd={}", n, fd);
        Ok(n)
    }

    /// 关闭文件
    pub fn close(fd: usize) -> AxResult {
        log::debug!("VfsOps::close: fd={}", fd);
        
        let mut table = FILE_TABLE.lock();
        if fd >= table.len() {
            return Err(AxError::BadState);
        }
        
        table[fd] = None;
        
        log::trace!("File closed: fd={}", fd);
        Ok(())
    }

    /// lseek: 移动文件读写指针
    pub fn lseek(fd: usize, offset: i64, whence: i32) -> AxResult<usize> {
        log::trace!("VfsOps::lseek: fd={}, offset={}, whence={}", fd, offset, whence);
        
        let mut table = FILE_TABLE.lock();
        let file_wrapper = table.get_mut(fd)
            .and_then(|opt| opt.as_mut())
            .ok_or(AxError::BadState)?;
        
        file_wrapper.seek(offset, whence)
    }

    /// fstat: 获取文件状态
    pub fn fstat(fd: usize) -> AxResult<axfs::api::FileMetadata> {
        log::trace!("VfsOps::fstat: fd={}", fd);
        
        let table = FILE_TABLE.lock();
        let file_wrapper = table.get(fd)
            .and_then(|opt| opt.as_ref())
            .ok_or(AxError::BadState)?;
        
        file_wrapper.metadata()
    }

    /// dup: 复制文件描述符
    pub fn dup(old_fd: usize) -> AxResult<usize> {
        log::trace!("VfsOps::dup: old_fd={}", old_fd);
        
        let mut table = FILE_TABLE.lock();
        let file_wrapper = table.get(old_fd)
            .and_then(|opt| opt.as_ref())
            .ok_or(AxError::BadState)?;
        
        // 创建新的包装器（共享底层文件）
        // 注意：这是简化实现，实际应该共享 File 引用
        let new_fd = table.len();
        let new_wrapper = FileWrapper {
            inner: file_wrapper.inner.clone()?,
            offset: file_wrapper.offset,
            flags: file_wrapper.flags,
        };
        table.push(Some(new_wrapper));
        
        Ok(new_fd)
    }

    /// dup2: 复制文件描述符到指定位置
    pub fn dup2(old_fd: usize, new_fd: usize) -> AxResult<usize> {
        log::trace!("VfsOps::dup2: old_fd={}, new_fd={}", old_fd, new_fd);
        
        let mut table = FILE_TABLE.lock();
        
        // 获取源文件
        let file_wrapper = table.get(old_fd)
            .and_then(|opt| opt.as_ref())
            .ok_or(AxError::BadState)?;
        
        let new_wrapper = FileWrapper {
            inner: file_wrapper.inner.clone()?,
            offset: file_wrapper.offset,
            flags: file_wrapper.flags,
        };
        
        // 扩展表大小
        while table.len() <= new_fd {
            table.push(None);
        }
        
        // 关闭旧的 new_fd
        table[new_fd] = Some(new_wrapper);
        
        Ok(new_fd)
    }

    /// mkdir: 创建目录
    pub fn mkdir(path: &str, mode: u32) -> AxResult {
        log::debug!("VfsOps::mkdir: {}, mode={:#o}", path, mode);
        axfs::api::create_dir(path)?;
        
        // 触发目录创建事件
        let watcher = unotify::get_watcher();
        let event = unotify::NotifyEvent::new(
            unotify::EventType::Create,
            path.to_string(),
        );
        watcher.trigger(event);
        
        Ok(())
    }

    /// getdents64: 读取目录项
    pub fn getdents64(fd: usize, buf: &mut [u8]) -> AxResult<usize> {
        log::trace!("VfsOps::getdents64: fd={}, buflen={}", fd, buf.len());
        
        // linux_dirent64 结构
        #[repr(C)]
        struct LinuxDirent64 {
            d_ino: u64,
            d_off: i64,
            d_reclen: u16,
            d_type: u8,
            // d_name 是可变长度的，不在这里定义
        }
        
        const DT_UNKNOWN: u8 = 0;
        const DT_REG: u8 = 8;
        const DT_DIR: u8 = 4;
        
        // 当前简化实现：返回空目录
        // 完整实现需要 axfs 支持目录迭代 API
        // 
        // 示例伪代码：
        // let table = FILE_TABLE.lock();
        // let file_wrapper = table.get(fd).ok_or(AxError::BadState)?;
        // let dir_iter = file_wrapper.inner.read_dir()?;
        // 
        // let mut offset = 0;
        // for entry in dir_iter {
        //     let name = entry.name();
        //     let reclen = calculate_reclen(name.len());
        //     if offset + reclen > buf.len() { break; }
        //     fill_dirent64(&mut buf[offset..], entry);
        //     offset += reclen;
        // }
        // Ok(offset)
        
        log::warn!("getdents64: Returning empty directory (not fully implemented)");
        Ok(0) // 返回 0 表示目录结束
    }

    /// unlink: 删除文件或目录
    pub fn unlink(path: &str) -> AxResult {
        log::debug!("VfsOps::unlink: {}", path);
        axfs::api::remove_file(path)?;
        
        // 触发文件删除事件
        let watcher = unotify::get_watcher();
        let event = unotify::NotifyEvent::new(
            unotify::EventType::Delete,
            path.to_string(),
        );
        watcher.trigger(event);
        
        Ok(())
    }
}
