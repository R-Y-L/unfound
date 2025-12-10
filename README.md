# Unfound OS

基于 ArceOS 的创新型操作系统 - 通过宏扩展实现智能文件系统

## 🎯 核心设计理念

采用**编译时宏扩展**而非运行时钩子，将 Unfound 的创新特性无缝集成到 ArceOS 文件系统中，实现零开销抽象。

## ✨ 创新特性

### 1. UNotify - 轻量级文件事件通知
- **零线程开销**：事件队列直接存储在内核态
- **四种事件类型**：`Create`, `Modify`, `Delete`, `Access`
- **高效事件传递**：环形缓冲区设计，支持批量读取
- **实时监控**：`pending_count()` 查询待处理事件数

### 2. UCache - 智能文件缓存
- **LRU 淘汰策略**：自动管理有限缓存空间
- **透明缓存**：读写操作自动命中/更新缓存
- **缓存失效**：文件删除时自动清理
- **统计信息**：实时监控命中率和性能

### 3. Unfound-FS - 宏扩展文件系统
- **`#[unfound_hook]`**：过程宏自动注入事件触发
- **扩展 API**：`fops_ext`, `api_ext` 提供增强功能
- **零侵入**：保持 ArceOS 原始代码不变
- **编译时优化**：宏展开后性能等同手写代码

## 📁 项目结构

```
unfound/
├── unfound-macros/          # 过程宏实现
│   └── src/lib.rs           # #[unfound_hook] 宏定义
│
├── unfound-fs/              # 扩展文件系统模块
│   └── src/lib.rs           # 封装 axfs + UNotify + UCache
│
├── xmodules/                # Unfound 扩展模块
│   ├── unotify/             # 文件事件通知系统
│   ├── ucache/              # 智能文件缓存
│   └── uvfs/                # VFS 抽象层（可选）
│
├── apps/
│   └── unfound_fs_test/     # 功能测试程序
│
└── arceos/                  # ArceOS 操作系统框架
    └── modules/axfs/        # 被扩展的文件系统
```

## 🚀 快速开始

### 构建测试程序

```bash
# 构建 unfound-fs 测试
make A=$PWD/apps/unfound_fs_test ARCH=riscv64 LOG=info BLK=y build

# 在 QEMU 中运行
make A=$PWD/apps/unfound_fs_test ARCH=riscv64 LOG=info BLK=y run
```

### 使用示例

```rust
use unfound_fs::{fops_ext, api_ext};

// 初始化 Unfound-FS (256 页缓存)
unfound_fs::init(256)?;

// 写入文件 (触发 Modify 事件 + 更新缓存)
fops_ext::write_file("/test.txt", b"Hello")?;

// 读取文件 (触发 Access 事件 + 缓存命中)
let data = fops_ext::read_file("/test.txt")?;

// 删除文件 (触发 Delete 事件 + 清除缓存)
api_ext::remove_file("/test.txt")?;

// 读取事件
if let Some(watcher) = unfound_fs::get_unotify_watcher() {
    let events = watcher.read_events(10);
    for event in events {
        println!("{:?}: {}", event.event_type, event.path);
    }
}
```

## 🏗️ 架构优势

### 传统钩子方案 vs 宏扩展方案

| 特性 | C FFI 钩子 | **宏扩展（Unfound）** |
|------|-----------|-------------------|
| 性能开销 | 函数调用 + 类型转换 | **零开销（编译时展开）** |
| 代码侵入 | 需修改 ArceOS 源码 | **完全无侵入** |
| 类型安全 | unsafe 跨语言调用 | **编译时检查** |
| 调试体验 | 难以追踪 | **清晰的宏展开** |
| 维护成本 | 高（需同步更新） | **低（独立演化）** |

### 宏扩展工作原理

```rust
// 用户代码
#[unfound_hook(event = "Access", cache_action = "Read")]
pub fn read_file(path: &str) -> Result<Vec<u8>> {
    // 原始实现
}

// 宏展开后
pub fn read_file(path: &str) -> Result<Vec<u8>> {
    // 1. 检查缓存
    if let Some(cached) = get_cache().get(path) {
        return Ok(cached);
    }
    
    // 2. 执行原始逻辑
    let result = { /* 原始实现 */ };
    
    // 3. 触发事件
    trigger_event(EventType::Access, path);
    
    result
}
```

## 📊 性能指标

### UCache 预期性能
- **缓存命中率**：顺序读取 > 80%，重复访问 > 95%
- **延迟降低**：缓存命中时减少 70-90% I/O 延迟
- **内存占用**：256 页 (1MB) 默认配置，可动态调整

### UNotify 特性
- **事件延迟**：< 100μs（内核态直接写入队列）
- **队列容量**：1024 事件（可配置）
- **批量读取**：一次最多获取 N 个事件

## 🛠️ 开发路线图

- [x] UNotify 事件系统核心实现
- [x] UCache LRU 缓存实现
- [x] Unfound-Macros 过程宏框架
- [x] Unfound-FS 扩展模块封装
- [x] 基础功能测试程序
- [ ] 性能基准测试
- [ ] 与 ArceOS 上游同步
- [ ] 支持更多事件类型 (Rename, Chmod 等)
- [ ] 缓存策略优化 (LFU, ARC)

## 📚 技术文档

### 关键组件

#### unfound-macros
- `#[unfound_hook]`: 函数级宏，注入事件和缓存逻辑
- `#[derive(UnfoundTracked)]`: 结构体级宏，自动实现追踪 trait

#### unfound-fs
- `init(cache_pages)`: 初始化 UNotify 和 UCache
- `fops_ext`: 扩展的文件操作 API (open/read/write)
- `api_ext`: 扩展的目录操作 API (create_dir/remove_file)

#### xmodules/unotify
- `EventType`: Create | Modify | Delete | Access
- `UNotifyWatcher`: 事件监视器，管理队列
- `trigger()`: 触发事件，`read_events()`: 读取事件

#### xmodules/ucache
- `UCache`: LRU 缓存管理器
- `get(path)`: 查询缓存，`put(path, data)`: 更新缓存
- `invalidate(path)`: 使缓存失效

## 🤝 参考项目

- [ArceOS](https://github.com/rcore-os/arceos) - 组件化操作系统框架
- [StarryX](https://github.com/Starry-OS/Starry) - POSIX 兼容 OS
- [inotify](https://man7.org/linux/man-pages/man7/inotify.7.html) - Linux 文件事件通知

## 📝 License

MIT License - 详见 LICENSE 文件

