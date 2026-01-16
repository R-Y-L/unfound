//! UnfoundOS 内核级交互式内存分配器测试框架 
//!
//! 核心特性:
//! - 多分配器支持 (Buddy/Bitmap/Hybrid)
//! - 运行时分配器切换
//! - 交互式命令行界面
//! - 分配器性能对比
//!
//! 运行方式:
//!   make run A=examples/allocators ARCH=riscv64 FEATURES="buddy,bitmap,hybrid"

#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[cfg(feature = "axstd")]
use axstd::{println, print};
#[cfg(feature = "axstd")]
extern crate alloc;
#[cfg(feature = "axstd")]
use alloc::vec::Vec;
#[cfg(feature = "axstd")]
use alloc::string::String;

#[cfg(not(feature = "axstd"))]
use std::{println, print, vec::Vec, string::String};

use axalloc::GlobalPage;
use axalloc::allocators::PageAllocator;
use core::sync::atomic::{AtomicU64, Ordering};

const PAGE_SIZE: usize = 4096;

// =============================================================================
// CPU 周期计数器
// =============================================================================

#[cfg(all(feature = "axstd", target_arch = "riscv64"))]
fn rdtsc() -> u64 {
    let cnt: u64;
    unsafe { core::arch::asm!("rdtime {}", out(reg) cnt); }
    cnt
}

#[cfg(all(feature = "axstd", target_arch = "aarch64"))]
fn rdtsc() -> u64 {
    let cnt: u64;
    unsafe { core::arch::asm!("mrs {}, cntvct_el0", out(reg) cnt); }
    cnt
}

#[cfg(all(feature = "axstd", target_arch = "x86_64"))]
fn rdtsc() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}

#[cfg(not(feature = "axstd"))]
fn rdtsc() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

// =============================================================================
// 简易随机数生成器
// =============================================================================

struct SimpleRng { state: u64 }

impl SimpleRng {
    fn new(seed: u64) -> Self { Self { state: seed } }
    
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }
    
    fn next_usize(&mut self, max: usize) -> usize {
        (self.next_u64() as usize) % max.max(1)
    }
    
    fn next_bool(&mut self, prob: u32) -> bool {
        (self.next_u64() % 100) < prob as u64
    }
}

// =============================================================================
// 测试统计
// =============================================================================

struct TestStats {
    total: usize,
    success: usize,
    total_alloc_cycles: u64,
    total_dealloc_cycles: u64,
    min_alloc: u64,
    max_alloc: u64,
}

impl TestStats {
    fn new() -> Self {
        Self {
            total: 0, success: 0,
            total_alloc_cycles: 0, total_dealloc_cycles: 0,
            min_alloc: u64::MAX, max_alloc: 0,
        }
    }
    
    fn record_alloc(&mut self, cycles: u64) {
        self.total_alloc_cycles += cycles;
        self.min_alloc = self.min_alloc.min(cycles);
        self.max_alloc = self.max_alloc.max(cycles);
    }
    
    fn avg_alloc(&self) -> u64 {
        if self.success > 0 { self.total_alloc_cycles / self.success as u64 } else { 0 }
    }
    
    fn avg_dealloc(&self) -> u64 {
        if self.success > 0 { self.total_dealloc_cycles / self.success as u64 } else { 0 }
    }
    
    fn success_rate(&self) -> u32 {
        if self.total > 0 { ((self.success * 100) / self.total) as u32 } else { 0 }
    }
    
    fn throughput(&self) -> u64 {
        let total = self.total_alloc_cycles + self.total_dealloc_cycles;
        if total > 0 { (self.success as u64 * 1_000_000) / total } else { 0 }
    }
}

// =============================================================================
// 分配器管理器 - 核心运行时切换逻辑
// =============================================================================

/// 当前活跃的分配器类型
#[derive(Clone, Copy, PartialEq)]
enum AllocatorType {
    System,  // 系统默认 GlobalPage
    Buddy,
    Bitmap,
    Hybrid,
}

impl AllocatorType {
    fn name(&self) -> &'static str {
        match self {
            Self::System => "System (GlobalPage)",
            Self::Buddy => "Buddy",
            Self::Bitmap => "Bitmap", 
            Self::Hybrid => "Hybrid",
        }
    }
}

/// 分配器管理器 - 封装运行时切换逻辑
struct AllocatorManager {
    current: AllocatorType,
    // 各分配器的独立内存池
    #[cfg(feature = "buddy")]
    buddy: Option<axalloc::allocators::BuddyAllocator>,
    #[cfg(feature = "bitmap")]
    bitmap: Option<axalloc::allocators::BitmapAllocator>,
    #[cfg(feature = "hybrid")]
    hybrid: Option<axalloc::allocators::HybridAllocator>,
    pool_start: usize,
    pool_size: usize,
}

impl AllocatorManager {
    fn new() -> Self {
        Self {
            current: AllocatorType::System,
            #[cfg(feature = "buddy")]
            buddy: None,
            #[cfg(feature = "bitmap")]
            bitmap: None,
            #[cfg(feature = "hybrid")]
            hybrid: None,
            pool_start: 0,
            pool_size: 0,
        }
    }
    
    /// 初始化内存池（为自定义分配器预留）
    fn init_pool(&mut self, size_mb: usize) -> bool {
        let num_pages = (size_mb * 1024 * 1024) / PAGE_SIZE;
        match GlobalPage::alloc_contiguous(num_pages, PAGE_SIZE) {
            Ok(page) => {
                self.pool_start = page.start_vaddr().as_usize();
                self.pool_size = num_pages * PAGE_SIZE;
                core::mem::forget(page);
                println!("  内存池初始化: {}MB @ 0x{:x}", size_mb, self.pool_start);
                true
            }
            Err(_) => {
                println!("  [错误] 无法分配 {}MB 内存池", size_mb);
                false
            }
        }
    }
    
    /// 切换到指定分配器（只在首次时初始化，后续切换仅更改当前指针）
    fn switch_to(&mut self, target: AllocatorType) -> bool {
        if target == AllocatorType::System {
            self.current = AllocatorType::System;
            println!("  切换到系统分配器 (GlobalPage)");
            return true;
        }
        
        if self.pool_size == 0 {
            println!("  [错误] 请先初始化内存池 (pool <size_mb>)");
            return false;
        }
        
        match target {
            #[cfg(feature = "buddy")]
            AllocatorType::Buddy => {
                // 如果已初始化，直接切换
                if self.buddy.is_some() {
                    self.current = AllocatorType::Buddy;
                    println!("  切换到 Buddy 分配器 (已初始化)");
                    return true;
                }
                // 首次初始化
                let alloc = axalloc::allocators::BuddyAllocator::new();
                if alloc.init(self.pool_start, self.pool_size).is_ok() {
                    self.buddy = Some(alloc);
                    self.current = AllocatorType::Buddy;
                    println!("  切换到 Buddy 分配器 ({}MB 内存池)", self.pool_size / 1024 / 1024);
                    return true;
                }
            }
            #[cfg(feature = "bitmap")]
            AllocatorType::Bitmap => {
                // 如果已初始化，直接切换
                if self.bitmap.is_some() {
                    self.current = AllocatorType::Bitmap;
                    println!("  切换到 Bitmap 分配器 (已初始化)");
                    return true;
                }
                
                // Bitmap 现在可以使用完整内存池
                let pages = self.pool_size / PAGE_SIZE;
                println!("  初始化 Bitmap 分配器 ({} 页, {}MB)...", pages, self.pool_size / 1024 / 1024);
                let start_time = rdtsc();
                let alloc = axalloc::allocators::BitmapAllocator::new();
                
                if alloc.init(self.pool_start, self.pool_size).is_ok() {
                    let elapsed = rdtsc().wrapping_sub(start_time);
                    self.bitmap = Some(alloc);
                    self.current = AllocatorType::Bitmap;
                    println!("  初始化完成: {} cycles ({} cycles/page)", elapsed, elapsed / pages as u64);
                    return true;
                }
                println!("  [错误] Bitmap 初始化失败");
            }
            #[cfg(feature = "hybrid")]
            AllocatorType::Hybrid => {
                // 如果已初始化，直接切换
                if self.hybrid.is_some() {
                    self.current = AllocatorType::Hybrid;
                    println!("  切换到 Hybrid 分配器 (已初始化)");
                    return true;
                }
                // 首次初始化
                let alloc = axalloc::allocators::HybridAllocator::new();
                if alloc.init(self.pool_start, self.pool_size).is_ok() {
                    self.hybrid = Some(alloc);
                    self.current = AllocatorType::Hybrid;
                    println!("  切换到 Hybrid 分配器 ({}MB 内存池)", self.pool_size / 1024 / 1024);
                    return true;
                }
            }
            _ => {}
        }
        
        #[allow(unreachable_code)]
        {
            println!("  [错误] 分配器未编译或初始化失败");
            false
        }
    }
    
    /// 使用当前分配器分配页面
    fn alloc_pages(&self, num_pages: usize) -> Result<usize, ()> {
        match self.current {
            AllocatorType::System => {
                if num_pages == 1 {
                    GlobalPage::alloc().map(|p| {
                        let addr = p.start_vaddr().as_usize();
                        core::mem::forget(p);
                        addr
                    }).map_err(|_| ())
                } else {
                    GlobalPage::alloc_contiguous(num_pages, PAGE_SIZE).map(|p| {
                        let addr = p.start_vaddr().as_usize();
                        core::mem::forget(p);
                        addr
                    }).map_err(|_| ())
                }
            }
            #[cfg(feature = "buddy")]
            AllocatorType::Buddy => {
                if let Some(ref alloc) = self.buddy {
                    alloc.alloc_pages(num_pages, PAGE_SIZE).map_err(|_| ())
                } else { Err(()) }
            }
            #[cfg(feature = "bitmap")]
            AllocatorType::Bitmap => {
                if let Some(ref alloc) = self.bitmap {
                    alloc.alloc_pages(num_pages, PAGE_SIZE).map_err(|_| ())
                } else { Err(()) }
            }
            #[cfg(feature = "hybrid")]
            AllocatorType::Hybrid => {
                if let Some(ref alloc) = self.hybrid {
                    alloc.alloc_pages(num_pages, PAGE_SIZE).map_err(|_| ())
                } else { Err(()) }
            }
            #[allow(unreachable_patterns)]
            _ => Err(()),
        }
    }
    
    /// 使用当前分配器释放页面
    fn dealloc_pages(&self, addr: usize, num_pages: usize) {
        match self.current {
            AllocatorType::System => {
                // System: 这里简化处理，实际需要重建 GlobalPage
            }
            #[cfg(feature = "buddy")]
            AllocatorType::Buddy => {
                if let Some(ref alloc) = self.buddy {
                    alloc.dealloc_pages(addr, num_pages);
                }
            }
            #[cfg(feature = "bitmap")]
            AllocatorType::Bitmap => {
                if let Some(ref alloc) = self.bitmap {
                    alloc.dealloc_pages(addr, num_pages);
                }
            }
            #[cfg(feature = "hybrid")]
            AllocatorType::Hybrid => {
                if let Some(ref alloc) = self.hybrid {
                    alloc.dealloc_pages(addr, num_pages);
                }
            }
            #[allow(unreachable_patterns)]
            _ => {}
        }
    }
    
    /// 获取当前分配器统计信息
    fn get_stats(&self) -> (f64, usize) {
        match self.current {
            #[cfg(feature = "buddy")]
            AllocatorType::Buddy => {
                if let Some(ref alloc) = self.buddy {
                    return alloc.get_stats();
                }
            }
            #[cfg(feature = "bitmap")]
            AllocatorType::Bitmap => {
                if let Some(ref alloc) = self.bitmap {
                    return alloc.get_stats();
                }
            }
            #[cfg(feature = "hybrid")]
            AllocatorType::Hybrid => {
                if let Some(ref alloc) = self.hybrid {
                    return alloc.get_stats();
                }
            }
            _ => {}
        }
        (0.0, 0)
    }
    
    /// 列出可用分配器
    fn list_available(&self) {
        println!("\n可用分配器:");
        #[cfg(feature = "buddy")]
        println!("  [ ] buddy   - 伙伴系统分配器");
        #[cfg(feature = "bitmap")]
        println!("  [ ] bitmap  - 位图分配器");
        #[cfg(feature = "hybrid")]
        println!("  [ ] hybrid  - 混合策略分配器");
        
        #[cfg(not(any(feature = "buddy", feature = "bitmap", feature = "hybrid")))]
        println!("  (未启用任何分配器)");
    }
}

// =============================================================================
// 测试函数
// =============================================================================

/// 基础性能测试
fn run_basic_test(mgr: &AllocatorManager, iterations: usize) -> TestStats {
    println!("\n--- 基础性能测试 ({} 次迭代) ---", iterations);
    
    let mut stats = TestStats::new();
    let mut rng = SimpleRng::new(rdtsc());
    let sizes = [1, 2, 4, 8, 16];
    let mut allocated: Vec<(usize, usize)> = Vec::new();
    
    for _ in 0..iterations {
        let size = sizes[rng.next_usize(sizes.len())];
        stats.total += 1;
        
        let start = rdtsc();
        if let Ok(addr) = mgr.alloc_pages(size) {
            let elapsed = rdtsc().wrapping_sub(start);
            stats.success += 1;
            stats.record_alloc(elapsed);
            allocated.push((addr, size));
        }
        
        if rng.next_bool(30) && !allocated.is_empty() {
            let idx = rng.next_usize(allocated.len());
            let (addr, size) = allocated.swap_remove(idx);
            let start = rdtsc();
            mgr.dealloc_pages(addr, size);
            stats.total_dealloc_cycles += rdtsc().wrapping_sub(start);
        }
    }
    
    for (addr, size) in allocated {
        mgr.dealloc_pages(addr, size);
    }
    
    println!("  分配器: {}", mgr.current.name());
    println!("  成功率: {}% ({}/{})", stats.success_rate(), stats.success, stats.total);
    println!("  分配: avg={} cycles, min={}, max={}", 
             stats.avg_alloc(), 
             if stats.min_alloc == u64::MAX { 0 } else { stats.min_alloc },
             stats.max_alloc);
    println!("  释放: avg={} cycles", stats.avg_dealloc());
    println!("  吞吐量: {} ops/M cycles", stats.throughput());
    
    stats
}

/// 内存读写验证测试
fn run_rw_test(mgr: &AllocatorManager) {
    println!("\n--- 内存读写验证测试 ---");
    
    print!("  单页 R/W: ");
    if let Ok(addr) = mgr.alloc_pages(1) {
        let ptr = addr as *mut u8;
        let pattern: u8 = 0xAB;
        
        unsafe {
            for i in 0..PAGE_SIZE { *ptr.add(i) = pattern; }
        }
        
        let mut ok = true;
        unsafe {
            for i in 0..PAGE_SIZE {
                if *ptr.add(i) != pattern { ok = false; break; }
            }
        }
        
        mgr.dealloc_pages(addr, 1);
        println!("{}", if ok { "[PASS]" } else { "[FAIL]" });
    } else {
        println!("[SKIP]");
    }
    
    print!("  8页边界 R/W: ");
    if let Ok(addr) = mgr.alloc_pages(8) {
        let ptr = addr as *mut u8;
        let mut ok = true;
        
        unsafe {
            for p in 0..8 {
                *ptr.add(p * PAGE_SIZE) = p as u8;
                *ptr.add(p * PAGE_SIZE + PAGE_SIZE - 1) = (p + 100) as u8;
            }
        }
        
        unsafe {
            for p in 0..8 {
                if *ptr.add(p * PAGE_SIZE) != p as u8 ||
                   *ptr.add(p * PAGE_SIZE + PAGE_SIZE - 1) != (p + 100) as u8 {
                    ok = false; break;
                }
            }
        }
        
        mgr.dealloc_pages(addr, 8);
        println!("{}", if ok { "[PASS]" } else { "[FAIL]" });
    } else {
        println!("[SKIP]");
    }
}

/// 碎片化测试
fn run_frag_test(mgr: &AllocatorManager) {
    println!("\n--- 碎片化测试 ---");
    
    let mut rng = SimpleRng::new(67890);
    let sizes = [1, 2, 4, 8, 16];
    let mut allocated: Vec<(usize, usize)> = Vec::new();
    
    for round in 1..=5 {
        for _ in 0..10 {
            let size = sizes[rng.next_usize(sizes.len())];
            if let Ok(addr) = mgr.alloc_pages(size) {
                allocated.push((addr, size));
            }
        }
        
        let to_release = allocated.len() * 40 / 100;
        for _ in 0..to_release {
            if !allocated.is_empty() {
                let idx = rng.next_usize(allocated.len());
                let (addr, size) = allocated.swap_remove(idx);
                mgr.dealloc_pages(addr, size);
            }
        }
        
        let (frag, free) = mgr.get_stats();
        println!("  轮次 {}: 持有 {} 块, 碎片率 {:.1}%, 空闲 {}KB",
                 round, allocated.len(), frag * 100.0, free / 1024);
    }
    
    print!("  碎片化后分配32页: ");
    if let Ok(addr) = mgr.alloc_pages(32) {
        println!("[PASS]");
        mgr.dealloc_pages(addr, 32);
    } else {
        println!("[FAIL]");
    }
    
    for (addr, size) in allocated {
        mgr.dealloc_pages(addr, size);
    }
}

/// 压力测试
fn run_stress_test(mgr: &AllocatorManager, ops: usize) {
    println!("\n--- 压力测试 ({} 操作) ---", ops);
    
    let mut rng = SimpleRng::new(99999);
    let mut stats = TestStats::new();
    let mut allocated: Vec<(usize, usize)> = Vec::new();
    let sizes = [1, 2, 4, 8, 16, 32];
    
    for i in 0..ops {
        let should_alloc = allocated.len() < 200 && (allocated.is_empty() || rng.next_bool(60));
        
        if should_alloc {
            let size = sizes[rng.next_usize(sizes.len())];
            stats.total += 1;
            let start = rdtsc();
            if let Ok(addr) = mgr.alloc_pages(size) {
                stats.success += 1;
                stats.record_alloc(rdtsc().wrapping_sub(start));
                allocated.push((addr, size));
            }
        } else if !allocated.is_empty() {
            let idx = rng.next_usize(allocated.len());
            let (addr, size) = allocated.swap_remove(idx);
            let start = rdtsc();
            mgr.dealloc_pages(addr, size);
            stats.total_dealloc_cycles += rdtsc().wrapping_sub(start);
        }
        
        if (i + 1) % (ops / 5).max(1) == 0 {
            println!("  进度: {}/{}, 持有: {} 块", i + 1, ops, allocated.len());
        }
    }
    
    for (addr, size) in allocated {
        mgr.dealloc_pages(addr, size);
    }
    
    println!("  结果: 成功率 {}%, 分配 {} cycles, 吞吐 {} ops/M",
             stats.success_rate(), stats.avg_alloc(), stats.throughput());
}

/// 分配器对比测试
fn run_compare_test(mgr: &mut AllocatorManager) {
    println!("\n========== 分配器性能对比 ==========");
    
    let mut results: Vec<(&str, TestStats)> = Vec::new();
    let iterations = 100;  // 减少迭代次数加快测试
    
    // 测试各分配器（共享同一个内存池）
    #[cfg(feature = "buddy")]
    {
        if mgr.switch_to(AllocatorType::Buddy) {
            results.push(("Buddy", run_basic_test(mgr, iterations)));
        }
    }
    
    // Bitmap 现在使用 Box 分配，不会栈溢出
    #[cfg(feature = "bitmap")]
    {
        if mgr.switch_to(AllocatorType::Bitmap) {
            results.push(("Bitmap", run_basic_test(mgr, iterations)));
        }
    }
    
    #[cfg(feature = "hybrid")]
    {
        if mgr.switch_to(AllocatorType::Hybrid) {
            results.push(("Hybrid", run_basic_test(mgr, iterations)));
        }
    }
    
    println!("\n┌─────────────────────────────────────────────────────────┐");
    println!("│                   性能对比汇总                          │");
    println!("├──────────┬──────────┬──────────┬──────────┬─────────────┤");
    println!("│ 分配器   │ 成功率   │ 分配     │ 释放     │ 吞吐量      │");
    println!("├──────────┼──────────┼──────────┼──────────┼─────────────┤");
    
    for (name, stats) in &results {
        println!("│ {:8} │ {:>6}% │ {:>6} c │ {:>6} c │ {:>7}/M c │",
                 name, stats.success_rate(), stats.avg_alloc(), 
                 stats.avg_dealloc(), stats.throughput());
    }
    
    println!("└──────────┴──────────┴──────────┴──────────┴─────────────┘");
}

// =============================================================================
// 命令解析
// =============================================================================

fn parse_command(input: &str, mgr: &mut AllocatorManager) -> bool {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() { return true; }
    
    match parts[0] {
        "help" | "h" | "?" => print_help(),
        "list" | "ls" => {
            mgr.list_available();
            println!("\n当前: {}", mgr.current.name());
        }
        "pool" => {
            let size = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(16);
            mgr.init_pool(size);
        }
        "switch" | "use" => {
            if let Some(name) = parts.get(1) {
                let target = match *name {
                    "buddy" => AllocatorType::Buddy,
                    "bitmap" => AllocatorType::Bitmap,
                    "hybrid" => AllocatorType::Hybrid,
                    _ => { println!("未知分配器: {}", name); return true; }
                };
                mgr.switch_to(target);
            } else {
                println!("用法: switch <buddy|bitmap|hybrid>");
            }
        }
        "basic" => {
            let iters = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(100);
            run_basic_test(mgr, iters);
        }
        "rw" => run_rw_test(mgr),
        "frag" => run_frag_test(mgr),
        "stress" => {
            let ops = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(500);
            run_stress_test(mgr, ops);
        }
        "compare" | "cmp" => {
            if mgr.pool_size == 0 {
                println!("请先初始化内存池: pool <size_mb>");
            } else {
                run_compare_test(mgr);
            }
        }
        "all" => {
            println!("\n运行全部测试...");
            run_basic_test(mgr, 100);
            run_rw_test(mgr);
            run_frag_test(mgr);
            run_stress_test(mgr, 500);
        }
        "stats" => {
            let (frag, free) = mgr.get_stats();
            println!("当前分配器: {}", mgr.current.name());
            println!("碎片率: {:.2}%", frag * 100.0);
            println!("空闲内存: {} KB", free / 1024);
        }
        "auto" => {
            // 调用完整的自动演示
            run_auto_demo(mgr);
        }
        "exit" | "quit" | "q" => {
            println!("退出测试框架");
            return false;
        }
        "" => {}
        _ => println!("未知命令: {}. 输入 'help' 查看帮助", parts[0]),
    }
    true
}

fn print_help() {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║          UnfoundOS 分配器测试框架 - 命令帮助             ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║ 分配器管理:                                              ║");
    println!("║   list              列出可用分配器                       ║");
    println!("║   pool <size_mb>    初始化内存池 (默认 16MB)             ║");
    println!("║   switch <name>     切换分配器 (buddy/bitmap/hybrid)       ║");
    println!("║   stats             显示当前分配器统计                   ║");
    println!("║                                                          ║");
    println!("║ 测试命令:                                                ║");
    println!("║   basic [n]         基础性能测试 (n次迭代, 默认100)      ║");
    println!("║   rw                内存读写验证                         ║");
    println!("║   frag              碎片化测试                           ║");
    println!("║   stress [n]        压力测试 (n次操作, 默认500)          ║");
    println!("║   compare           对比所有分配器性能                   ║");
    println!("║   all               运行全部测试                         ║");
    println!("║   auto              自动演示模式                         ║");
    println!("║                                                          ║");
    println!("║ 其他:                                                    ║");
    println!("║   help              显示此帮助                           ║");
    println!("║   exit              退出                                 ║");
    println!("╚══════════════════════════════════════════════════════════╝");
}

fn print_banner() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║    UnfoundOS 内核级交互式分配器测试框架 v3.0              ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  支持运行时分配器切换 | 多分配器性能对比                  ║");
    println!("║  输入 'help' 查看命令 | 输入 'auto' 自动演示              ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
}

// =============================================================================
// 主入口
// =============================================================================

#[cfg(feature = "axstd")]
fn read_line() -> String {
    use axstd::io::{stdin, Read};
    let mut buf = [0u8; 128];
    let mut line = String::new();
    
    // 逐字符读取直到换行
    loop {
        let mut byte = [0u8; 1];
        match stdin().read(&mut byte) {
            Ok(1) => {
                let ch = byte[0] as char;
                if ch == '\n' || ch == '\r' {
                    break;
                }
                line.push(ch);
                // 回显字符
                print!("{}", ch);
            }
            Ok(0) => {
                // EOF - 等待更多输入
                for _ in 0..1000 { core::hint::spin_loop(); }
                continue;
            }
            _ => break,
        }
    }
    println!(); // 换行
    line
}

#[cfg(not(feature = "axstd"))]
fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line
}

/// 非交互式自动测试模式
fn run_auto_demo(mgr: &mut AllocatorManager) {
    println!("\n=== 自动演示模式 (非交互式) ===\n");
    
    // 1. 初始化内存池 - 使用 2MB 以加快 Bitmap 初始化
    println!("【阶段 1】初始化内存池");
    if !mgr.init_pool(2) {  // 2MB = 512 页，Bitmap 初始化更快
        println!("内存池初始化失败，跳过自定义分配器测试");
        return;
    }
    
    // 2. 测试 Buddy 分配器
    #[cfg(feature = "buddy")]
    {
        println!("\n【阶段 2】切换到 Buddy 分配器");
        if mgr.switch_to(AllocatorType::Buddy) {
            run_basic_test(mgr, 100);
            run_rw_test(mgr);
            run_frag_test(mgr);
        }
    }
    
    // 3. 测试 Bitmap 分配器
    #[cfg(feature = "bitmap")]
    {
        println!("\n【阶段 3】切换到 Bitmap 分配器");
        if mgr.switch_to(AllocatorType::Bitmap) {
            run_basic_test(mgr, 100);
            run_rw_test(mgr);
        }
    }
    
    // 4. 测试 Hybrid 分配器
    #[cfg(feature = "hybrid")]
    {
        println!("\n【阶段 4】切换到 Hybrid 分配器");
        if mgr.switch_to(AllocatorType::Hybrid) {
            run_basic_test(mgr, 100);
            run_rw_test(mgr);
        }
    }
    
    // 5. 性能对比
    println!("\n【阶段 5】分配器性能对比");
    run_compare_test(mgr);
    
    println!("\n=== 自动演示完成 ===");
}

#[cfg_attr(feature = "axstd", unsafe(no_mangle))]
fn main() {
    print_banner();
    
    let mut mgr = AllocatorManager::new();
    mgr.list_available();
    
    #[cfg(feature = "axstd")]
    {
        use axstd::io::{stdin, Read};
        
        println!("\n按 Enter 进入交互模式，或等待自动运行演示...");
        print!("unfound> ");
        
        // 尝试读取一个字符
        let mut first_char: Option<char> = None;
        let mut buf = [0u8; 1];
        
        // 尝试读取（在真实终端中会阻塞等待输入）
        // 在管道模式下，read 会立即返回 0 或 EOF
        match stdin().read(&mut buf) {
            Ok(n) if n > 0 => {
                let ch = buf[0] as char;
                if ch != '\n' && ch != '\r' {
                    first_char = Some(ch);
                    print!("{}", ch);  // 回显第一个字符
                }
                println!();
            }
            _ => {
                // 无法读取输入，运行自动演示
                println!("\n[自动模式] 运行演示测试...");
            }
        }
        
        if first_char.is_some() || buf[0] == b'\n' || buf[0] == b'\r' {
            // 交互模式 - 如果有第一个字符，需要处理它
            println!("\n输入 'help' 查看命令，'auto' 自动演示，'exit' 退出\n");
            
            // 如果第一个字符不是换行，需要继续读取完整命令
            if let Some(ch) = first_char {
                print!("unfound> {}", ch);
                let rest = read_line();
                // 构建完整命令：第一个字符 + 剩余输入
                let mut cmd = String::new();
                cmd.push(ch);
                cmd.push_str(&rest);
                if !parse_command(&cmd, &mut mgr) {
                    println!("\n测试框架已退出。");
                    axstd::process::exit(0);
                }
            }
            
            loop {
                print!("unfound> ");
                let line = read_line();
                if !parse_command(&line, &mut mgr) {
                    break;
                }
            }
        } else {
            // 自动演示模式
            run_auto_demo(&mut mgr);
        }
        
        println!("\n测试框架已退出。");
        axstd::process::exit(0);
    }
    
    #[cfg(not(feature = "axstd"))]
    {
        println!("\n输入 'help' 查看命令，'auto' 自动演示，'exit' 退出\n");
        
        loop {
            print!("unfound> ");
            let line = read_line();
            if !parse_command(&line, &mut mgr) {
                break;
            }
        }
    }
    
    println!("测试框架已退出。");
}
