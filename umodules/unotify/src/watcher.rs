/// 文件监控器实现

use alloc::vec::Vec;
use alloc::collections::{VecDeque, BTreeMap};
use alloc::string::String;
use spin::RwLock;
use crate::event::{NotifyEvent, EventType};
use axerrno::{AxResult, AxError};

/// 监控描述符
pub type WatchDescriptor = i32;

/// 监控条目
#[derive(Debug, Clone)]
struct WatchEntry {
    wd: WatchDescriptor,
    path: String,
    mask: u32,  // 事件掩码
}

/// 监控器
pub struct FileWatcher {
    event_queue: RwLock<VecDeque<NotifyEvent>>,
    watches: RwLock<BTreeMap<WatchDescriptor, WatchEntry>>,
    next_wd: RwLock<WatchDescriptor>,
    max_events: usize,
}

impl FileWatcher {
    pub fn new() -> Self {
        Self {
            event_queue: RwLock::new(VecDeque::new()),
            watches: RwLock::new(BTreeMap::new()),
            next_wd: RwLock::new(1),
            max_events: 1024,
        }
    }

    /// 添加监控路径
    pub fn add_watch(&self, path: &str, mask: u32) -> AxResult<WatchDescriptor> {
        let mut next_wd = self.next_wd.write();
        let wd = *next_wd;
        *next_wd += 1;
        
        let entry = WatchEntry {
            wd,
            path: String::from(path),
            mask,
        };
        
        self.watches.write().insert(wd, entry);
        log::info!("Added watch: wd={}, path={}, mask={:#x}", wd, path, mask);
        Ok(wd)
    }

    /// 移除监控
    pub fn remove_watch(&self, wd: WatchDescriptor) -> AxResult {
        if self.watches.write().remove(&wd).is_some() {
            log::info!("Removed watch: wd={}", wd);
            Ok(())
        } else {
            Err(AxError::NotFound)
        }
    }

    /// 检查路径是否被监控，并返回匹配的掩码
    fn check_watch(&self, path: &str) -> Option<u32> {
        let watches = self.watches.read();
        for entry in watches.values() {
            // 简单的前缀匹配
            if path.starts_with(&entry.path) {
                return Some(entry.mask);
            }
        }
        None
    }

    /// 触发事件（带路径检查）
    pub fn trigger(&self, event: NotifyEvent) {
        // 检查路径是否被监控
        if let Some(mask) = self.check_watch(&event.path) {
            let event_bit = event.event_type as u32;
            if mask & event_bit != 0 {
                let mut queue = self.event_queue.write();
                if queue.len() >= self.max_events {
                    queue.pop_front(); // 丢弃最旧事件
                }
                log::debug!("File event: {:?} on {}", event.event_type, event.path);
                queue.push_back(event);
            }
        }
    }

    /// 无条件触发事件（用于测试）
    pub fn trigger_unchecked(&self, event: NotifyEvent) {
        let mut queue = self.event_queue.write();
        if queue.len() >= self.max_events {
            queue.pop_front();
        }
        log::debug!("File event (unchecked): {:?}", event);
        queue.push_back(event);
    }

    /// 读取事件
    pub fn read_events(&self, max_count: usize) -> Vec<NotifyEvent> {
        let mut queue = self.event_queue.write();
        let count = max_count.min(queue.len());
        queue.drain(..count).collect()
    }

    /// 获取待处理事件数量
    pub fn pending_count(&self) -> usize {
        self.event_queue.read().len()
    }
}
