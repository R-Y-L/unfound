# Unfound OS

基于 ArceOS 使用 Rust 开发的操作系统，具备良好的模块化设计与用户态支持能力。

## 项目简介

本项目是基于 [ArceOS](https://github.com/arceos-org/arceos) 开发的操作系统，采用 Rust 语言实现。系统继承了 ArceOS 的模块化架构设计，并在进程管理、内存管理、文件系统方面进行了重要改进，主要包括：

文件系统：
- **统一文件接口**：为不同文件系统类型提供统一的抽象接口
- **动态挂载机制**：支持运行时动态挂载和卸载文件系统

## 项目结构

```
unfound/
├── api/                      # API 层
│   ├── arceos_api/          # ArceOS 核心 API
│   ├── arceos_posix_api/     # POSIX 兼容 API
│   └── axfeat/              # 特性定义
├── modules/                 # 核心模块
│   ├── axfs/                # 文件系统模块
│   │   ├── src/
│   │   │   ├── root.rs      # 根目录和挂载管理
│   │   │   ├── fops.rs      # 文件操作统一接口
│   │   │   ├── api/         # 文件系统 API
│   │   │   └── fs/          # 具体文件系统实现
│   │   └── mounts.rs        # 挂载点管理
│   ├── axalloc/             # 内存分配器
│   ├── axhal/               # 硬件抽象层
│   ├── axtask/              # 任务管理
│   ├── axnet/               # 网络栈
│   ├── axsync/              # 同步原语
│   └── ...                  # 其他核心模块
├── crates/                  # 外部依赖和文件系统实现
│   └── axfs_crates/         # 文件系统相关 crate
│       ├── axfs_vfs/        # 虚拟文件系统接口（VFS）
│       ├── axfs_ramfs/      # 内存文件系统
│       ├── axfs_devfs/      # 设备文件系统
│       └── axfs_procfs/     # 进程文件系统
├── ulib/                    # 用户态库
│   ├── axstd/              # 标准库接口
│   └── axlibc/             # C 标准库接口
├── examples/                # 示例程序
│   ├── helloworld/         # Hello World 示例
│   ├── shell/              # Shell 示例
│   ├── httpserver/         # HTTP 服务器示例
│   └── ...
├── configs/                 # 配置文件
├── scripts/                 # 构建脚本
├── tools/                   # 工具程序
└── doc/                     # 文档

```

## 主要改进

### 1. 统一文件接口

系统通过虚拟文件系统（VFS）层为不同的文件系统类型（如 Ext4、FAT、RamFS、DevFS、ProcFS 等）提供了统一的抽象接口。

#### 实现方式

**VFS 抽象层** (`crates/axfs_crates/axfs_vfs/`)：
- 定义了 `VfsOps` trait，所有文件系统必须实现此 trait
- 定义了 `VfsNodeOps` trait，所有文件/目录节点必须实现此 trait
- 提供了统一的文件操作接口：`read_at()`, `write_at()`, `lookup()`, `create()`, `remove()` 等

**文件操作封装** (`modules/axfs/src/fops.rs`)：
- `File` 结构体：封装了文件节点，提供统一的文件读写接口
- `Directory` 结构体：封装了目录节点，提供目录遍历和管理接口
- `OpenOptions`：统一的文件打开选项配置

**POSIX API 层** (`api/arceos_posix_api/src/imp/fs.rs`)：
- 实现了 POSIX 标准的文件系统调用（`open`, `read`, `write`, `stat`, `mount` 等）
- 通过 `FileLike` trait 统一处理文件和目录的文件描述符操作
- 支持文件描述符表管理，统一管理所有打开的文件和目录

#### 优势

- **类型安全**：利用 Rust 的类型系统确保文件系统实现的正确性
- **易于扩展**：新增文件系统类型只需实现 `VfsOps` 和 `VfsNodeOps` trait
- **统一接口**：上层应用无需关心底层文件系统类型，使用统一的 API

### 2. 动态挂载机制

系统支持在运行时动态挂载和卸载文件系统，实现了灵活的存储管理。

#### 实现方式

**挂载点管理** (`modules/axfs/src/root.rs`)：
- `RootDirectory` 结构体：管理根文件系统和所有挂载点
- `MountPoint` 结构体：存储挂载点路径和对应的文件系统实例
- 使用 `RwLock<Vec<MountPoint>>` 管理挂载点列表，支持并发访问

**挂载流程**：
```rust
pub fn mount(&self, path: &'static str, fs: Arc<dyn VfsOps>) -> AxResult {
    // 1. 验证挂载路径
    // 2. 在主文件系统中创建挂载点目录
    // 3. 调用文件系统的 mount() 方法
    // 4. 将挂载点添加到挂载列表
}
```

**路径查找机制**：
- `lookup_mounted_fs()` 方法：根据路径查找对应的文件系统
- 使用最长匹配算法：找到与路径匹配的最长挂载点
- 支持嵌套挂载：可以在已挂载的文件系统上再次挂载

**初始化时的自动挂载** (`init_rootfs()`)：
- 根据编译时特性自动挂载不同的文件系统：
  - `/dev`：设备文件系统（DevFS）
  - `/tmp`：内存文件系统（RamFS）
  - `/proc`：进程文件系统（ProcFS）
  - `/sys`：系统文件系统（SysFS）

## 快速开始

### 构建和运行

```bash
# 使用 Docker（推荐）
docker build -t arceos -f Dockerfile .
docker run -it -v $(pwd):/arceos -w /arceos arceos bash

# 在容器中构建和运行
make A=examples/helloworld ARCH=aarch64 run
```

### 手动构建

```bash
# 安装依赖
cargo install cargo-binutils axconfig-gen cargo-axplat

# 构建和运行
make A=examples/helloworld ARCH=aarch64 LOG=info run
```

## 许可证

本项目采用多重许可证：
- GPL-3.0-or-later
- Apache-2.0
- MulanPSL-2.0

详见各 LICENSE 文件。

## 参考

- [ArceOS 官方仓库](https://github.com/arceos-org/arceos)
- [ArceOS 文档](https://arceos-org.github.io/arceos/)
