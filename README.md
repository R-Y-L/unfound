# Unfound

Unfound 是 ArceOS 模块化微内核的个人定制分支，保留上游架构并作为调度器、驱动和平台适配的实验沙箱。本分支演进较快，版本间可能出现不兼容变更。

内存管理工作（本分支重点）：
- 调整物理页分配器与伙伴系统接口，澄清与硬件无关的内存抽象。
- 引入按域隔离（内核/用户/设备）的虚拟地址空间布局，支撑安全性实验。
- 增强内存诊断日志与统计，便于问题复现与性能画像。
- 补充并调优页面回收策略，评估吞吐与尾延迟表现。

Memory management focus (EN): refining allocator/buddy interfaces, per-domain VA layouts, richer MM telemetry, and reclamation tuning.

## 支持的目标平台
- 架构：x86_64、aarch64、riscv64、loongarch64
- 默认平台：QEMU pc-q35（x86_64）、QEMU virt（aarch64/riscv64/loongarch64）

## 项目目录结构
- modules：内核组件（内存、调度、驱动、网络、文件系统、日志等）
- api：应用接口（axfeat、arceos_api、arceos_posix_api）
- ulib：用户态库（axstd、axlibc）
- examples：子系统示例与冒烟测试
- configs：平台配置
- scripts、tools：构建与 QEMU 运行脚本
- doc：设计文档与图表

## 快速开始

### 前置依赖
- 安装 rust-toolchain.toml 指定的工具链（rustup）
- 安装辅助工具：
	```bash
	cargo install cargo-binutils axconfig-gen cargo-axplat
	```
- 安装 QEMU：
	```bash
	# Ubuntu/Debian
	sudo apt-get install qemu-system
	# macOS
	brew install qemu
	```
- 可选（C 应用）：`sudo apt install libclang-dev`，并安装 aarch64/riscv64/x86_64/loongarch64 的 musl 交叉工具链，将其 bin 目录加入 PATH。

### 编译并运行示例
```bash
# 在仓库根目录
make A=examples/helloworld ARCH=aarch64 LOG=info run
```

常用参数：`SMP=<cpus>` 设置核数；`NET=y` 启用 virtio-net；`BLK=y` 启用 virtio-blk；`GRAPHIC=y` 启用 virtio-gpu。完整列表见 Makefile。

### 构建自定义 Rust 应用
1. 创建 `no_std`、`no_main` 包，添加依赖：
	 ```toml
	 [dependencies]
	 axstd = { path = "/path/to/unfound/ulib/axstd", features = ["..."] }
	 ```
2. 用 `#[unsafe(no_mangle)]` 标记入口，按 `axstd`（类似 Rust `std`）编程。
3. 在本仓库环境构建运行：
	 ```bash
	 make -C /path/to/unfound A=$(pwd) ARCH=<arch> run
	 ```

### 构建自定义 C 应用
1. 在应用目录添加 `axbuild.mk`（列出对象文件）和可选 `features.txt`（每行一个功能）。
2. 使用同样的 make 命令，令 `A` 指向应用目录。

## 其他说明
- 默认配置输出到 `.axconfig.toml`，可用 `OUT_CONFIG` 重定向。
- 构建产物位于 `target/`，可用 `TARGET_DIR` 覆盖。
- 本仓库紧跟上游 ArceOS，同步补丁直接提交于此。

## 开源协议
与上游一致：GPL-3.0-or-later、Apache-2.0、MulanPSL-2.0。