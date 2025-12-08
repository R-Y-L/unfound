# Unfound OS

基于ArceOS的创新型操作系统

## 创新特性

### 1. UCache - 智能页缓存
- **LRU淘汰策略**：自动管理内存页
- **自适应预读**：检测顺序/随机访问模式
- **缓存统计**：实时监控命中率

### 2. UNotify - 文件事件通知
- **轻量级设计**：无额外线程开销
- **事件类型**：CREATE, MODIFY, DELETE, ACCESS
- **用户态队列**：高效事件传递

### 3. UVFS - 统一VFS抽象
- **FileLike trait**：统一文件接口
- **DentryCache**：目录项缓存
- **VFS层透明集成**

## 项目结构

```
unfound/
├── arceos/          # ArceOS子模块
├── uapi/            # 系统调用层
│   ├── syscall/     # 系统调用实现
│   └── interface/   # 用户态接口
├── ucore/           # 核心抽象
│   ├── process.rs   # 进程管理
│   └── memory.rs    # 内存管理
├── xmodules/        # 扩展模块
│   ├── uvfs/        # VFS实现
│   ├── ucache/      # 页缓存（创新）
│   └── unotify/     # 文件通知（创新）
└── apps/            # 测试应用
    └── tests/       # 性能测试
```

## 构建和运行

```bash
# 构建（需要Rust nightly）
make build

# 在QEMU中运行
make run ARCH=riscv64

# 运行测试
make test
```

## 性能指标

### UCache预期性能
- 顺序读取缓存命中率：> 80%
- 随机读取延迟降低：30-50%
- 内存占用：256页（1MB默认配置）

### UNotify特性
- 事件延迟：< 1ms
- 队列容量：1024事件
- 零拷贝设计

## 开发路线图

- [x] 基础目录结构
- [x] UCache核心实现
- [x] UNotify事件系统
- [ ] 与ArceOS axfs集成
- [ ] 性能测试和优化
- [ ] POSIX兼容性增强

## 参考项目

- [ArceOS](https://github.com/rcore-os/arceos)
- [StarryX](https://github.com/Starry-OS/Starry)

## License

MIT
