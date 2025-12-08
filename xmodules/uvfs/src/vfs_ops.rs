/// VFS操作抽象层
use axerrno::{AxResult, AxError};
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use spin::Mutex;
use crate::FileWrapper;

extern crate ucache;
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

    /// 从文件读取，集成页缓存
    pub fn read(fd: usize, buf: &mut [u8]) -> AxResult<usize> {
        log::trace!("VfsOps::read: fd={}, len={}", fd, buf.len());
        
        // 获取文件包装器
        let mut table = FILE_TABLE.lock();
        let file_wrapper = table.get_mut(fd)
            .and_then(|opt| opt.as_mut())
            .ok_or(AxError::BadState)?;
        
        let offset = file_wrapper.offset;
        drop(table); // 释放锁
        
        // 使用页缓存读取
        let cache = ucache::get_cache();
        let mut total_read = 0;
        let mut current_offset = offset;
        
        while total_read < buf.len() {
            // 获取当前页
            match cache.get_page(fd, current_offset) {
                Ok(page) => {
                    let page_offset = current_offset % ucache::PAGE_SIZE;
                    let available = ucache::PAGE_SIZE - page_offset;
                    let to_copy = core::cmp::min(available, buf.len() - total_read);
                    
                    buf[total_read..total_read + to_copy]
                        .copy_from_slice(&page.data[page_offset..page_offset + to_copy]);
                    
                    total_read += to_copy;
                    current_offset += to_copy;
                }
                Err(_) => {
                    // 缓存未命中，直接从文件读取
                    let mut table = FILE_TABLE.lock();
                    let file_wrapper = table.get_mut(fd)
                        .and_then(|opt| opt.as_mut())
                        .ok_or(AxError::BadState)?;
                    
                    let n = file_wrapper.read(&mut buf[total_read..])?;
                    total_read += n;
                    break;
                }
            }
        }
        
        log::trace!("Read {} bytes from fd={}", total_read, fd);
        Ok(total_read)
    }

    /// 向文件写入，更新缓存并触发通知
    pub fn write(fd: usize, buf: &[u8]) -> AxResult<usize> {
        log::trace!("VfsOps::write: fd={}, len={}", fd, buf.len());
        
        // 直接写入文件（写穿策略）
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

    /// 关闭文件，清理缓存
    pub fn close(fd: usize) -> AxResult {
        log::debug!("VfsOps::close: fd={}", fd);
        
        let mut table = FILE_TABLE.lock();
        if fd >= table.len() {
            return Err(AxError::BadState);
        }
        
        table[fd] = None;
        
        // TODO: 清理该文件的所有缓存页
        
        log::trace!("File closed: fd={}", fd);
        Ok(())
    }
}
