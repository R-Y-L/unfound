#![no_std]

extern crate alloc;

mod vfs_ops;
mod file_wrapper;

pub use vfs_ops::VfsOps;
pub use file_wrapper::FileWrapper;

// 重新导出 unotify 的类型
pub use unotify::{NotifyEvent, EventType};

use axerrno::AxResult;

/// VFS模块初始化
pub fn init() -> AxResult {
    log::info!("Initializing unfound VFS...");
    axfs::init_filesystems();
    Ok(())
}
