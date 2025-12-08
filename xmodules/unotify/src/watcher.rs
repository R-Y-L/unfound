/// 文件监控器实现

use alloc::vec::Vec;
use alloc::collections::VecDeque;
use spin::RwLock;
use crate::event::{NotifyEvent, EventType};
use axerrno::AxResult;

/// 监控器
pub struct FileWatcher {
    event_queue: RwLock<VecDeque<NotifyEvent>>,
    max_events: usize,
}

impl FileWatcher {
    pub fn new() -> Self {
        Self {
            event_queue: RwLock::new(VecDeque::new()),
            max_events: 1024,
        }
    }

    /// 触发事件
    pub fn trigger(&self, event: NotifyEvent) {
        let mut queue = self.event_queue.write();
        if queue.len() >= self.max_events {
            queue.pop_front(); // 丢弃最旧事件
        }
        log::debug!("File event: {:?}", event);
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
