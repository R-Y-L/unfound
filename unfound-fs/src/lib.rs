//! Unfound 文件系统 - 扩展 ArceOS axfs 模块
//! 
//! 通过宏注入的方式集成 UNotify 和 UCache 功能

#![no_std]

extern crate alloc;

use spin::Mutex;
use alloc::sync::Arc;

// 重新导出核心类型
pub use unotify::{EventType, NotifyEvent, UNotifyWatcher};
pub use ucache::UCache;
pub use unfound_macros::{unfound_hook, UnfoundTracked};

// 重新导出 axfs 的所有公共接口
pub use axfs::*;

/// 全局 UNotify 监视器
static UNOTIFY_WATCHER: Mutex<Option<Arc<UNotifyWatcher>>> = Mutex::new(None);

/// 全局 UCache 实例
static UCACHE: Mutex<Option<Arc<UCache>>> = Mutex::new(None);

/// 初始化 Unfound 文件系统扩展
pub fn init(cache_pages: usize) -> Result<(), &'static str> {
    // 初始化 UNotify
    match unotify::init() {
        Ok(_) => {
            let watcher = unotify::get_watcher();
            *UNOTIFY_WATCHER.lock() = Some(watcher);
            log::info!("[Unfound-FS] UNotify initialized");
        }
        Err(e) => {
            log::error!("[Unfound-FS] Failed to initialize UNotify: {:?}", e);
            return Err("UNotify init failed");
        }
    }
    
    // 初始化 UCache
    match ucache::init(cache_pages) {
        Ok(_) => {
            if let Some(cache) = ucache::get_cache() {
                *UCACHE.lock() = Some(cache);
                log::info!("[Unfound-FS] UCache initialized with {} pages", cache_pages);
            }
        }
        Err(e) => {
            log::error!("[Unfound-FS] Failed to initialize UCache: {:?}", e);
            return Err("UCache init failed");
        }
    }
    
    Ok(())
}

/// 获取 UNotify 监视器
pub fn get_unotify_watcher() -> Option<Arc<UNotifyWatcher>> {
    UNOTIFY_WATCHER.lock().clone()
}

/// 获取 UCache 实例
pub fn get_ucache() -> Option<Arc<UCache>> {
    UCACHE.lock().clone()
}

/// Unfound 跟踪 trait
pub trait Tracked {
    fn on_access(&self);
    fn on_modify(&self);
}

/// 扩展的文件操作 API
pub mod fops_ext {
    use super::*;
    use axfs::fops::{File, OpenOptions};
    use axerrno::AxResult;
    use unfound_macros::unfound_hook;
    
    /// 打开文件 (带 UNotify 和 UCache)
    #[unfound_hook(event = "Access", path_param = "path")]
    pub fn open(path: &str, opts: &OpenOptions) -> AxResult<File> {
        axfs::fops::File::open(path, opts)
    }
    
    /// 读取文件 (带 ARC 缓存检查)
    pub fn read_file(path: &str) -> AxResult<alloc::vec::Vec<u8>> {
        use alloc::string::ToString;
        
        // 先检查 ARC 缓存
        if let Some(cache) = get_ucache() {
            if let Some(data) = cache.get(&path.to_string()) {
                log::debug!("[Unfound-FS] ARC Cache HIT: {}", path);
                
                // 触发 Access 事件
                if let Some(watcher) = get_unotify_watcher() {
                    watcher.trigger(NotifyEvent::new(
                        EventType::Access,
                        path.to_string()
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
