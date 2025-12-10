//! UNotify 功能测试

use unotify::{EventType, NotifyEvent, init, get_watcher};

#[test]
fn test_init() {
    assert!(init().is_ok(), "UNotify 初始化失败");
}

#[test]
fn test_trigger_and_read() {
    init().unwrap();
    let watcher = get_watcher();
    
    // 触发事件
    watcher.trigger(NotifyEvent::new(EventType::Create, "/test1.txt".into()));
    watcher.trigger(NotifyEvent::new(EventType::Modify, "/test2.txt".into()));
    watcher.trigger(NotifyEvent::new(EventType::Access, "/test3.txt".into()));
    watcher.trigger(NotifyEvent::new(EventType::Delete, "/test4.txt".into()));
    
    // 检查数量
    assert_eq!(watcher.pending_count(), 4, "事件数量不正确");
    
    // 读取事件
    let events = watcher.read_events(10);
    assert_eq!(events.len(), 4, "读取事件数量不正确");
    
    // 验证类型
    assert_eq!(events[0].event_type, EventType::Create);
    assert_eq!(events[1].event_type, EventType::Modify);
    assert_eq!(events[2].event_type, EventType::Access);
    assert_eq!(events[3].event_type, EventType::Delete);
    
    // 队列应该清空
    assert_eq!(watcher.pending_count(), 0, "队列未清空");
}

#[test]
fn test_batch_events() {
    init().unwrap();
    let watcher = get_watcher();
    
    // 批量触发
    for i in 0..100 {
        let path = format!("/file{}.txt", i);
        watcher.trigger(NotifyEvent::new(EventType::Access, path));
    }
    
    assert_eq!(watcher.pending_count(), 100, "批量触发失败");
    
    // 部分读取
    let events = watcher.read_events(50);
    assert_eq!(events.len(), 50, "批量读取数量错误");
    assert_eq!(watcher.pending_count(), 50, "剩余数量错误");
    
    // 读取剩余
    let remaining = watcher.read_events(100);
    assert_eq!(remaining.len(), 50, "剩余读取错误");
    assert_eq!(watcher.pending_count(), 0, "最终队列未清空");
}

#[test]
fn test_event_paths() {
    init().unwrap();
    let watcher = get_watcher();
    
    watcher.trigger(NotifyEvent::new(EventType::Create, "/path/to/file.txt".into()));
    
    let events = watcher.read_events(1);
    assert_eq!(events[0].path, "/path/to/file.txt", "路径不匹配");
}
