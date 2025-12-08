#![no_std]
//! UNotify - 轻量级文件变化通知模块
//! 
//! 创新特性：
//! - 事件驱动的文件监控
//! - 支持 IN_CREATE, IN_MODIFY, IN_DELETE 事件
//! - 用户态事件队列

extern crate alloc;

mod event;
mod watcher;

pub use event::{NotifyEvent, EventType};
pub use watcher::FileWatcher;

use axerrno::AxResult;
use spin::Mutex;
use alloc::sync::Arc;

static GLOBAL_WATCHER: Mutex<Option<Arc<FileWatcher>>> = Mutex::new(None);

/// 初始化文件监控
pub fn init() -> AxResult {
    log::info!("Initializing UNotify...");
    let watcher = Arc::new(FileWatcher::new());
    *GLOBAL_WATCHER.lock() = Some(watcher);
    Ok(())
}

/// 获取全局监控器
pub fn get_watcher() -> Arc<FileWatcher> {
    GLOBAL_WATCHER.lock().as_ref().unwrap().clone()
}
