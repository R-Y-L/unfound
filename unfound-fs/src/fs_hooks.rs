//! Unfound 文件系统集成
//! 
//! 为 axfs 文件操作添加事件通知功能

use axerrno::AxResult;
use alloc::vec::Vec;
use alloc::string::{String, ToString};

/// 触发文件事件的辅助函数
fn trigger_event(event_type: unotify::EventType, path: &str) {
    let watcher = unotify::get_watcher();
    let event = unotify::NotifyEvent::new(event_type, path.to_string());
    watcher.trigger(event);
}

/// 带事件通知的文件读取
pub fn read_file_with_notify(path: &str) -> AxResult<Vec<u8>> {
    use axfs::api::{File, read as axfs_read};
    
    // 触发 Access 事件
    trigger_event(unotify::EventType::Access, path);
    
    // 执行原始读取
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    axfs_read(&mut file, &mut buf)?;
    
    Ok(buf)
}

/// 带事件通知的文件写入
pub fn write_file_with_notify(path: &str, data: &[u8]) -> AxResult<usize> {
    use axfs::api::{File, OpenOptions, write as axfs_write};
    
    let is_new_file = !axfs::api::metadata(path).is_ok();
    
    // 打开文件
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    
    // 执行写入
    let n = axfs_write(&mut file, data)?;
    
    // 触发事件
    if is_new_file {
        trigger_event(unotify::EventType::Create, path);
    }
    trigger_event(unotify::EventType::Modify, path);
    
    Ok(n)
}

/// 带事件通知的文件创建
pub fn create_file_with_notify(path: &str) -> AxResult {
    use axfs::api::File;
    
    File::create(path)?;
    trigger_event(unotify::EventType::Create, path);
    
    Ok(())
}

/// 带事件通知的文件删除
pub fn remove_file_with_notify(path: &str) -> AxResult {
    use axfs::api::remove_file as axfs_remove_file;
    
    axfs_remove_file(path)?;
    trigger_event(unotify::EventType::Delete, path);
    
    Ok(())
}

/// 带事件通知的目录创建
pub fn create_dir_with_notify(path: &str) -> AxResult {
    use axfs::api::create_dir as axfs_create_dir;
    
    axfs_create_dir(path)?;
    trigger_event(unotify::EventType::Create, path);
    
    Ok(())
}

/// 带事件通知的目录删除
pub fn remove_dir_with_notify(path: &str) -> AxResult {
    use axfs::api::remove_dir as axfs_remove_dir;
    
    axfs_remove_dir(path)?;
    trigger_event(unotify::EventType::Delete, path);
    
    Ok(())
}

/// 带事件通知的文件重命名
pub fn rename_with_notify(old_path: &str, new_path: &str) -> AxResult {
    use axfs::api::rename as axfs_rename;
    
    axfs_rename(old_path, new_path)?;
    trigger_event(unotify::EventType::Delete, old_path);
    trigger_event(unotify::EventType::Create, new_path);
    
    Ok(())
}
