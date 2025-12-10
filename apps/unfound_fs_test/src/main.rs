//! Unfound-FS 功能测试
//! 测试 UNotify 事件触发和 UCache 缓存

#![no_std]
#![no_main]

#[macro_use]
extern crate axstd;

use axstd::println;
use unfound_fs::{fops_ext, api_ext, get_unotify_watcher};

#[no_mangle]
fn main() {
    println!("========================================");
    println!("  Unfound-FS 扩展文件系统测试");
    println!("========================================");
    println!();
    
    // 初始化 Unfound-FS
    println!("[初始化] 启动 Unfound-FS (256页缓存)...");
    match unfound_fs::init(256) {
        Ok(_) => println!("[初始化] ✓ Unfound-FS 初始化成功"),
        Err(e) => {
            println!("[初始化] ✗ 初始化失败: {}", e);
            return;
        }
    }
    println!();
    
    // 测试 1: 文件写入 (应触发 Modify 事件)
    println!("[测试 1] 写入文件...");
    let test_data = b"Hello from Unfound-FS!";
    match fops_ext::write_file("/test_unfound.txt", test_data) {
        Ok(_) => println!("[测试 1] ✓ 文件写入成功"),
        Err(e) => println!("[测试 1] ✗ 写入失败: {:?}", e),
    }
    check_events(1, "Modify");
    println!();
    
    // 测试 2: 第一次读取 (缓存未命中, 应触发 Access 事件)
    println!("[测试 2] 第一次读取文件 (缓存未命中)...");
    match fops_ext::read_file("/test_unfound.txt") {
        Ok(data) => {
            println!("[测试 2] ✓ 读取成功: {} 字节", data.len());
            if &data[..] == test_data {
                println!("[测试 2] ✓ 数据验证通过");
            } else {
                println!("[测试 2] ✗ 数据不匹配!");
            }
        }
        Err(e) => println!("[测试 2] ✗ 读取失败: {:?}", e),
    }
    check_events(1, "Access");
    println!();
    
    // 测试 3: 第二次读取 (缓存命中, 应触发 Access 事件)
    println!("[测试 3] 第二次读取文件 (缓存命中)...");
    match fops_ext::read_file("/test_unfound.txt") {
        Ok(data) => {
            println!("[测试 3] ✓ 读取成功: {} 字节 (from cache)", data.len());
        }
        Err(e) => println!("[测试 3] ✗ 读取失败: {:?}", e),
    }
    check_events(1, "Access");
    println!();
    
    // 测试 4: 创建目录 (应触发 Create 事件)
    println!("[测试 4] 创建目录...");
    match api_ext::create_dir("/test_dir") {
        Ok(_) => println!("[测试 4] ✓ 目录创建成功"),
        Err(e) => println!("[测试 4] ✗ 创建失败: {:?}", e),
    }
    check_events(1, "Create");
    println!();
    
    // 测试 5: 删除文件 (应触发 Delete 事件并清除缓存)
    println!("[测试 5] 删除文件...");
    match api_ext::remove_file("/test_unfound.txt") {
        Ok(_) => println!("[测试 5] ✓ 文件删除成功"),
        Err(e) => println!("[测试 5] ✗ 删除失败: {:?}", e),
    }
    check_events(1, "Delete");
    println!();
    
    // 测试 6: 再次读取 (应该失败,缓存已清除)
    println!("[测试 6] 尝试读取已删除的文件...");
    match fops_ext::read_file("/test_unfound.txt") {
        Ok(_) => println!("[测试 6] ✗ 不应该成功!"),
        Err(e) => println!("[测试 6] ✓ 正确失败: {:?}", e),
    }
    println!();
    
    println!("========================================");
    println!("  测试完成");
    println!("========================================");
}

/// 检查并显示 UNotify 事件
fn check_events(expected: usize, event_type: &str) {
    if let Some(watcher) = get_unotify_watcher() {
        let count = watcher.pending_count();
        println!("  [UNotify] 待处理事件: {} 个", count);
        
        if count > 0 {
            let events = watcher.read_events(count);
            for (i, event) in events.iter().enumerate() {
                println!("  [UNotify] 事件 {}: {:?} - {}", i+1, event.event_type, event.path);
            }
            
            if events.len() == expected {
                println!("  [UNotify] ✓ 事件数量正确 (预期 {} 个 {} 事件)", expected, event_type);
            } else {
                println!("  [UNotify] ✗ 事件数量错误 (预期 {}, 实际 {})", expected, events.len());
            }
        } else {
            println!("  [UNotify] ✗ 没有检测到事件!");
        }
    } else {
        println!("  [UNotify] ✗ 无法获取监视器");
    }
}
