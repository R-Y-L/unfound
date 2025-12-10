/// 文件事件定义

use alloc::string::String;

/// 事件类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    Create = 1,
    Modify = 2,
    Delete = 4,
    Access = 8,
}

/// 通知事件
#[derive(Debug, Clone)]
pub struct NotifyEvent {
    pub event_type: EventType,
    pub path: String,
    pub timestamp: u64,
}

impl NotifyEvent {
    pub fn new(event_type: EventType, path: String) -> Self {
        Self {
            event_type,
            path,
            timestamp: 0, // TODO: 获取系统时间戳
        }
    }
}
