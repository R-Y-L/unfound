//! Unfound 文件系统 - 扩展 ArceOS axfs 模块
//! 
//! 通过封装的方式集成 UNotify 和 UCache 功能

#![no_std]

extern crate alloc;

mod fs_hooks;

// 重新导出核心类型
pub use unotify::{EventType, NotifyEvent, FileWatcher, WatchDescriptor};
pub use unfound_macros::{unfound_hook, UnfoundTracked};

// 重新导出文件系统钩子
pub use fs_hooks::*;

/// 获取 UNotify 监视器
pub fn get_unotify_watcher() -> unotify::FileWatcher {
    unotify::get_watcher()
}

/// Unfound 跟踪 trait
pub trait Tracked {
    fn on_access(&self);
    fn on_modify(&self);
}
                    ));
                }
                
                return Ok(data);
            }
        }
        
        // 缓存未命中,读取文件
        log::debug!("[Unfound-FS] ARC Cache MISS: {}", path);
        let opts = OpenOptions::new().read(true);
        let mut file = axfs::fops::File::open(path, &opts)?;
        
        use axio::Read;
        let mut buf = alloc::vec::Vec::new();
        file.read_to_end(&mut buf)?;
        
        // 更新 ARC 缓存
        if let Some(cache) = get_ucache() {
            cache.put(path.to_string(), buf.clone());
        }
        
        // 触发 Access 事件
        if let Some(watcher) = get_unotify_watcher() {
            watcher.trigger(NotifyEvent::new(
                EventType::Access,
                path.to_string()
            ));
        }
        
        Ok(buf)
    }
    
    /// 写入文件 (带 ARC 缓存更新)
    pub fn write_file(path: &str, data: &[u8]) -> AxResult<()> {
        use alloc::string::ToString;
        
        let opts = OpenOptions::new().write(true).create(true).truncate(true);
        let mut file = axfs::fops::File::open(path, &opts)?;
        
        use axio::Write;
        file.write_all(data)?;
        
        // 更新 ARC 缓存
        if let Some(cache) = get_ucache() {
            cache.put(path.to_string(), data.to_vec());
        }
        
        // 触发 Modify 事件
        if let Some(watcher) = get_unotify_watcher() {
            watcher.trigger(NotifyEvent::new(
                EventType::Modify,
                path.to_string()
            ));
        }
        
        Ok(())
    }
}

/// 扩展的目录操作 API
pub mod api_ext {
    use super::*;
    use axerrno::AxResult;
    
    /// 创建目录 (带 UNotify)
    pub fn create_dir(path: &str) -> AxResult {
        let result = axfs::api::create_dir(path);
        
        if result.is_ok() {
            // 触发 Create 事件
            if let Some(watcher) = get_unotify_watcher() {
                watcher.trigger(NotifyEvent::new(
                    EventType::Create,
                    alloc::string::String::from(path)
                ));
            }
        }
        
        result
    }
    
    /// 删除文件 (带 UNotify 和 ARC 缓存清理)
    pub fn remove_file(path: &str) -> AxResult {
        use alloc::string::ToString;
        
        let result = axfs::api::remove_file(path);
        
        if result.is_ok() {
            // 清除 ARC 缓存
            if let Some(cache) = get_ucache() {
                cache.invalidate(&path.to_string());
            }
            
            // 触发 Delete 事件
            if let Some(watcher) = get_unotify_watcher() {
                watcher.trigger(NotifyEvent::new(
                    EventType::Delete,
                    path.to_string()
                ));
            }
        }
        
        result
    }
    
    /// 删除目录 (带 UNotify)
    pub fn remove_dir(path: &str) -> AxResult {
        let result = axfs::api::remove_dir(path);
        
        if result.is_ok() {
            // 触发 Delete 事件
            if let Some(watcher) = get_unotify_watcher() {
                watcher.trigger(NotifyEvent::new(
                    EventType::Delete,
                    alloc::string::String::from(path)
                ));
            }
        }
        
        result
    }
}
