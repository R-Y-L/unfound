# UnfoundOS 内存分配器测试框架

本文档详细介绍了 UnfoundOS 中用于评估内存分配器的全维度测试框架。

## 一、框架概述

该测试框架提供模块化、可扩展的测试能力，用于评估内存分配器的性能、碎片情况、稳定性及时间维度表现，核心支持：

- **基础性能测试**：分配成功率、分配 / 释放耗时、吞吐量
- **碎片测试**：外部 / 内部碎片率、元数据开销
- **稳定性测试**：分配失败率、内存泄漏检测、多线程竞争开销
- **时间维度测试**：长期运行模拟（1 小时、6 小时、24 小时）
- **运行时切换测试**：分配器切换耗时与兼容性
- **内核兼容测试**：适配嵌入式 / 内核环境的 no_std 测试能力
- **可视化分析与报告生成**：自动生成Markdown格式的测试报告，包含性能指标、碎片分析、时间序列数据等，支持多分配器对比视图

## 二、快速开始

### 2.1 命令行运行测试

bash

运行

```bash
# 运行 Buddy 分配器全量测试套件
cargo run --bin allocator_test --features "buddy std" all

# 运行指定类型测试
cargo run --bin allocator_test --features "buddy std" basic          # 基础性能测试
cargo run --bin allocator_test --features "buddy std" fragmentation  # 碎片测试
cargo run --bin allocator_test --features "buddy std" stability      # 稳定性测试
cargo run --bin allocator_test --features "buddy std" time-dimension # 时间维度测试

# 对比多个分配器性能
cargo run --bin allocator_test --features "buddy bitmap hybrid std" -- --compare

# 自定义配置级别运行
cargo run --bin allocator_test --features "buddy std" -c large all
```

### 2.2 命令行参数说明

plaintext

```plaintext
使用方式: allocator_test [可选参数] [测试负载]

测试负载:
  all, complete        运行全量测试套件（默认）
  basic                运行基础性能测试
  fragmentation        运行碎片测试
  stability            运行稳定性测试
  time-dimension       运行时间维度测试
  legacy               运行初始负载测试

可选参数:
  -a, --allocator NAME   指定分配器（buddy/bitmap/hybrid）
  -c, --config LEVEL     配置级别（small/medium/large）
  --compare              对比所有可用分配器的性能
  -v, --verbose          启用详细输出
  -h, --help             显示帮助信息
```

## 三、程序化调用

### 3.1 基础性能测试

rust

运行

```rust
use axalloc::tests::{run_basic_performance_test, BasicPerfConfig};
use axalloc::allocators::BuddyAllocator;

// 初始化 Buddy 分配器（256MB 内存池）
let allocator = BuddyAllocator::new();
allocator.init(0x1000, 256 * 1024 * 1024).unwrap();

// 执行基础性能测试
let config = BasicPerfConfig::default();
let metrics = run_basic_performance_test(&allocator, &config);
println!("{}", metrics.to_report());
```

### 3.2 全量测试套件

rust

运行

```rust
use axalloc::tests::{run_complete_test, TestConfig};
use axalloc::allocators::BuddyAllocator;

// 初始化 Buddy 分配器（256MB 内存池）
let allocator = BuddyAllocator::new();
allocator.init(0x1000, 256 * 1024 * 1024).unwrap();

// 执行全量测试（中等配置）
let config = TestConfig::medium();
let report = run_complete_test(&allocator, &config);
println!("{}", report.to_full_report());
```

### 3.3 时间维度测试

rust

运行

```rust
use axalloc::tests::{run_time_dimension_test, TimeDimensionConfig};
use axalloc::allocators::BuddyAllocator;

// 初始化 Buddy 分配器（1GB 内存池）
let allocator = BuddyAllocator::new();
allocator.init(0x1000, 1024 * 1024 * 1024).unwrap();

// 执行24小时全量时间维度测试
let config = TimeDimensionConfig::full_24h();
let results = run_time_dimension_test(&allocator, &config);
println!("{}", results.to_report());
```

### 3.4 内核兼容测试（no_std）

rust

运行

```rust
use axalloc::tests::kernel_tests::{
    kernel_basic_test,
    kernel_fragmentation_test,
    kernel_leak_test,
    run_all_kernel_tests,
};

// 运行所有内核环境测试
run_all_kernel_tests(&allocator);

// 或单独运行指定测试
let result = kernel_basic_test(&allocator, 1000);
result.log_result();
```

## 四、测试配置

### 4.1 预设配置级别

- `small`：快速验证（64MB 内存池，1000 次分配）
- `medium`：标准测试（256MB 内存池，10000 次分配）
- `large`：压力测试（1GB 内存池，100000 次分配）

### 4.2 自定义配置

rust

运行

```rust
use axalloc::tests::{TestConfig, BasicPerfConfig};

// 初始化默认配置并修改
let mut config = TestConfig::default();
config.memory_pool_size = 512 * 1024 * 1024; // 512MB 内存池
config.basic_perf.num_allocations = 50000;   // 5万次分配
config.basic_perf.measure_individual_times = true; // 统计单次耗时
config.verbose = true; // 启用详细输出
```

## 五、输出指标示例

### 5.1 基础性能报告

plaintext

```plaintext
【基础性能】
- 分配成功率：64.6%（尝试10000次，成功6461次）
- 平均分配时间：569.36ns
- 平均释放时间：1.28μs
- 吞吐量：909496 次/秒
- P50延迟：509.00ns，P99延迟：1.40μs
```

> **注意**：时间单位根据量级自动选择：
> - < 1μs → 显示为 **ns**（纳秒）
> - 1μs ~ 1ms → 显示为 **μs**（微秒）
> - ≥ 1ms → 显示为 **ms**（毫秒）

### 5.1.1 分配器性能对比

| 分配器 | 成功率 | 平均分配 | 平均释放 | 吞吐量 | P50 | P99 |
|--------|--------|----------|----------|--------|-----|-----|
| **Buddy** | 64.6% | 569ns | 1.28μs | 909K/s | 509ns | 1.4μs |
| **Bitmap** | 66.3% | 16.4μs | 339ns | 55K/s | 5.7μs | 147μs |
| **Hybrid** | 73.6% | 157μs | 1.16μs | 13K/s | 86μs | 640μs |

> **说明**：上述数据基于 1GB 内存池、10000 次分配测试（混合分配/释放模式，30% 概率释放）

### 5.2 碎片报告（时间维度）

plaintext

```plaintext
【碎片与开销（时间维度）】
- 外部碎片率：
  - 运行0h：5.1%（总空闲200MB，最大连续190MB）
  - 运行1h：10.3%（总空闲180MB，最大连续161MB）
  - 运行6h：15.2%（总空闲200MB，最大连续169.6MB）
  - 运行24h：18.5%（总空闲190MB，最大连续155MB）
- 内部碎片率：
  - 运行0h：5.2%（已分配64MB，浪费3.3MB）
  - 运行6h：8.3%（已分配128MB，浪费10.6MB）
  - 运行24h：9.1%（已分配140MB，浪费12.7MB）
- 元数据占用：32KB（总内存1GB，占比0.003%）
```

### 5.3 稳定性报告（时间维度）

plaintext

```plaintext
【稳定性（时间维度）】
- 分配失败率（按大小+时间）：
  - 16KB：运行1h(1%) → 运行6h(5%) → 运行24h(7%)
  - 1MB：运行1h(10%) → 运行6h(30%) → 运行24h(35%)
- 内存泄漏率：0 B/s（循环100000次/持续24h运行）
- 多线程竞争开销：
  - 运行1h：1.2倍（4线程 vs 单线程）
  - 运行6h：1.3倍（4线程 vs 单线程）
  - 运行24h：1.4倍（4线程 vs 单线程）
- 性能衰减率：
  - 平均分配时间衰减：运行1h(0.12ms) → 运行24h(0.15ms)（衰减25%）
  - 吞吐量衰减：运行1h(15000次/秒) → 运行24h(12000次/秒)（衰减20%）
```

## 六、新增自定义分配器测试

测试框架兼容所有实现 `PageAllocator` trait 的分配器，接入步骤如下：

### 6.1 实现 PageAllocator 特质

rust

运行

```rust
pub trait PageAllocator: Send + Sync {
    // 返回分配器名称
    fn name(&self) -> &'static str;
    // 初始化分配器（起始虚拟地址、内存大小）
    fn init(&self, start_vaddr: usize, size: usize) -> Result<(), AllocError>;
    // 分配物理页（页数、对齐位数）
    fn alloc_pages(&self, num_pages: usize, align_pow2: usize) -> Result<usize, AllocError>;
    // 指定地址分配物理页
    fn alloc_pages_at(&self, start: usize, num_pages: usize, align_pow2: usize) -> Result<usize, AllocError>;
    // 释放物理页
    fn dealloc_pages(&self, pos: usize, num_pages: usize);
    // 获取统计信息（碎片率、总空闲字节数）
    fn get_stats(&self) -> (f64, usize);
}
```

### 6.2 接入测试框架

1. 实现上述 `PageAllocator` 特质；
2. 如需条件编译，在 Cargo.toml 中添加对应的 feature 标识；
3. 使用框架执行测试：

rust

运行

```rust
use axalloc::tests::{run_complete_test, TestConfig};

// 初始化自定义分配器（256MB 内存池）
let my_allocator = MyCustomAllocator::new();
my_allocator.init(0x1000, 256 * 1024 * 1024).unwrap();

// 执行中等配置的全量测试
let report = run_complete_test(&my_allocator, &TestConfig::medium());
println!("{}", report.to_full_report());
```

## 七、模块结构

plaintext

```plaintext
src/tests/
├── mod.rs              # 模块入口，导出核心接口
├── metrics.rs          # 测试结果数据结构定义
├── config.rs           # 测试配置参数定义
├── suites.rs           # 核心测试套件实现
├── time_dimension.rs   # 长期运行的时间维度测试
├── kernel_tests.rs     # 适配 no_std 的内核环境测试
├── allocator_tester.rs # 初始测试器（向下兼容）
└── workloads.rs        # 初始测试负载（向下兼容）
```

## 八、编译特性（Features）

- `std`：启用基于标准库的全功能测试
- `buddy`：启用伙伴系统分配器测试
- `bitmap`：启用位图分配器测试
- `hybrid`：启用混合策略分配器测试
- `runtime-switch`：启用运行时分配器切换测试
- `page-alloc-4g`：支持最大 4GB 内存池（默认，1M 页）
- `page-alloc-64g`：支持最大 64GB 内存池（16M 页）
- `page-alloc-256m`：支持最大 256MB 内存池（64K 页，用于嵌入式环境）

