//! Test module: Comprehensive allocator testing framework.
//!
//! This module provides a modular, extensible testing framework for memory
//! allocators. It supports testing any allocator that implements the
//! `PageAllocator` trait.
//!
//! ## Structure
//!
//! - `metrics`: Data structures for test results and reporting
//! - `config`: Test configuration parameters
//! - `suites`: Individual test suite implementations
//! - `workloads`: Legacy workload definitions (for compatibility)
//! - `allocator_tester`: Legacy tester (for compatibility)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use axalloc::tests::{run_complete_test, TestConfig};
//! use axalloc::allocators::BuddyAllocator;
//!
//! let allocator = BuddyAllocator::new();
//! allocator.init(0x1000, 64 * 1024 * 1024).unwrap();
//!
//! let config = TestConfig::medium();
//! let report = run_complete_test(&allocator, &config);
//! println!("{}", report.to_full_report());
//! ```

extern crate std;
extern crate alloc;

// New modular test framework
pub mod metrics;
pub mod config;
pub mod suites;
pub mod time_dimension;

// Kernel-compatible tests (no_std)
pub mod kernel_tests;

// Legacy modules (kept for backward compatibility)
pub mod allocator_tester;
pub mod workloads;

// Re-export commonly used items
pub use metrics::*;
pub use config::*;
pub use suites::*;
pub use time_dimension::*;
pub use kernel_tests::*;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;

use crate::allocators::PageAllocator;

#[cfg(feature = "buddy")]
use crate::allocators::BuddyAllocator;
#[cfg(feature = "bitmap")]
use crate::allocators::BitmapAllocator;
#[cfg(feature = "hybrid")]
use crate::allocators::HybridAllocator;

// =============================================================================
// High-level Test Entry Points
// =============================================================================

/// Run the complete test suite on the default allocator.
pub fn run_default_allocator_tests() {
    #[cfg(feature = "buddy")]
    let allocator: Box<dyn PageAllocator> = Box::new(BuddyAllocator::new());
    #[cfg(all(feature = "bitmap", not(feature = "buddy")))]
    let allocator: Box<dyn PageAllocator> = Box::new(BitmapAllocator::new());
    #[cfg(all(feature = "hybrid", not(feature = "buddy"), not(feature = "bitmap")))]
    let allocator: Box<dyn PageAllocator> = Box::new(HybridAllocator::new());
    
    #[cfg(not(any(feature = "buddy", feature = "bitmap", feature = "hybrid")))]
    {
        std::println!("No allocator feature enabled. Enable buddy, bitmap, or hybrid.");
        return;
    }
    
    #[cfg(any(feature = "buddy", feature = "bitmap", feature = "hybrid"))]
    {
        let config = TestConfig::medium();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        
        let report = run_complete_test(&*allocator, &config);
        std::println!("{}", report.to_full_report());
    }
}

/// Run tests on a specific allocator by name.
pub fn run_tests_by_name(name: &str, config: &TestConfig) -> Result<CompleteTestReport, &'static str> {
    #[cfg(feature = "runtime-switch")]
    {
        let allocator = crate::allocators::runtime::make_by_name(name)?;
        allocator.init(config.start_vaddr, config.memory_pool_size)
            .map_err(|_| "Failed to initialize allocator")?;
        Ok(run_complete_test(&*allocator, config))
    }
    
    #[cfg(not(feature = "runtime-switch"))]
    {
        match name {
            #[cfg(feature = "buddy")]
            "buddy" => {
                let allocator = BuddyAllocator::new();
                allocator.init(config.start_vaddr, config.memory_pool_size)
                    .map_err(|_| "Failed to initialize allocator")?;
                Ok(run_complete_test(&allocator, config))
            }
            #[cfg(feature = "bitmap")]
            "bitmap" => {
                let allocator = BitmapAllocator::new();
                allocator.init(config.start_vaddr, config.memory_pool_size)
                    .map_err(|_| "Failed to initialize allocator")?;
                Ok(run_complete_test(&allocator, config))
            }
            #[cfg(feature = "hybrid")]
            "hybrid" => {
                let allocator = HybridAllocator::new();
                allocator.init(config.start_vaddr, config.memory_pool_size)
                    .map_err(|_| "Failed to initialize allocator")?;
                Ok(run_complete_test(&allocator, config))
            }
            _ => Err("Unknown or disabled allocator"),
        }
    }
}

/// Compare multiple allocators.
pub fn compare_allocators(names: &[&str], config: &TestConfig) -> Vec<(String, CompleteTestReport)> {
    let mut results = Vec::new();
    
    for name in names {
        match run_tests_by_name(name, config) {
            Ok(report) => {
                results.push((String::from(*name), report));
            }
            Err(e) => {
                std::println!("Skipping {}: {}", name, e);
            }
        }
    }
    
    results
}

/// Print a comparison table of allocator results.
pub fn print_comparison_table(results: &[(String, CompleteTestReport)]) {
    std::println!("\n╔══════════════════════════════════════════════════════════════════════════╗");
    std::println!("║                          分配器性能对比表                                 ║");
    std::println!("╠══════════════╦═════════════╦═════════════╦═════════════╦═════════════════╣");
    std::println!("║    分配器    ║   成功率    ║  平均分配   ║  平均释放   ║     吞吐量      ║");
    std::println!("╠══════════════╬═════════════╬═════════════╬═════════════╬═════════════════╣");
    
    for (name, report) in results {
        std::println!(
            "║ {:^12} ║ {:>9.1}% ║ {:>9.2}ms ║ {:>9.2}ms ║ {:>13.0}/s ║",
            name,
            report.basic_performance.success_rate,
            report.basic_performance.avg_alloc_time_us / 1000.0,
            report.basic_performance.avg_dealloc_time_us / 1000.0,
            report.basic_performance.throughput_ops_per_sec
        );
    }
    
    std::println!("╚══════════════╩═════════════╩═════════════╩═════════════╩═════════════════╝");
}

// =============================================================================
// Legacy Entry Points (for backward compatibility)
// =============================================================================

/// Run allocator tests from command line arguments.
/// 
/// Usage: cargo run --bin allocator_test [OPTIONS] [WORKLOAD]
/// 
/// Workloads:
///   all, complete    Run complete test suite (default)
///   basic            Run basic performance tests only
///   fragmentation    Run fragmentation tests only
///   stability        Run stability tests only
///   legacy, small, large, mixed  Run legacy workloads
/// 
/// Options:
///   -a, --allocator NAME   Select allocator (buddy, bitmap, hybrid)
///   -c, --config LEVEL     Config level (small, medium, large)
///   --compare              Compare all available allocators
///   -h, --help             Show help
pub fn run_allocator_tests_from_cli() {
    let args: Vec<String> = std::env::args().collect();
    
    // Parse command line
    let mut workload = "all".to_string();
    let mut allocator_name = "default".to_string();
    let mut config_level = "medium".to_string();
    let mut compare_mode = false;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--workload" | "-w" => {
                if i + 1 < args.len() {
                    workload = args[i + 1].clone();
                    i += 1;
                }
            }
            "--allocator" | "-a" => {
                if i + 1 < args.len() {
                    allocator_name = args[i + 1].clone();
                    i += 1;
                }
            }
            "--config" | "-c" => {
                if i + 1 < args.len() {
                    config_level = args[i + 1].clone();
                    i += 1;
                }
            }
            "--compare" => {
                compare_mode = true;
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            other => {
                // Legacy: first positional arg is workload
                if i == 1 {
                    workload = other.to_string();
                }
            }
        }
        i += 1;
    }
    
    // Select config
    let config = match config_level.as_str() {
        "small" | "quick" => TestConfig::small(),
        "medium" | "default" => TestConfig::medium(),
        "large" | "stress" => TestConfig::large(),
        _ => TestConfig::medium(),
    };
    
    if compare_mode {
        // Compare all available allocators
        let names: Vec<&str> = vec![
            #[cfg(feature = "buddy")]
            "buddy",
            #[cfg(feature = "bitmap")]
            "bitmap",
            #[cfg(feature = "hybrid")]
            "hybrid",
        ];
        let results = compare_allocators(&names, &config);
        print_comparison_table(&results);
        for (_, report) in &results {
            std::println!("{}", report.to_full_report());
        }
    } else {
        match workload.as_str() {
            "all" | "complete" => {
                match run_tests_by_name(&allocator_name_or_default(&allocator_name), &config) {
                    Ok(report) => std::println!("{}", report.to_full_report()),
                    Err(e) => std::println!("Error: {}", e),
                }
            }
            "basic" => {
                run_basic_only(&allocator_name, &config);
            }
            "fragmentation" => {
                run_fragmentation_only(&allocator_name, &config);
            }
            "stability" => {
                run_stability_only(&allocator_name, &config);
            }
            "legacy" | "small" | "large" | "mixed" => {
                // Run legacy workloads
                run_legacy_workload(&workload);
            }
            _ => {
                std::println!("Unknown workload: {}", workload);
                print_help();
            }
        }
    }
}

fn allocator_name_or_default(name: &str) -> &str {
    if name == "default" {
        #[cfg(feature = "hybrid")]
        return "hybrid";
        #[cfg(all(feature = "buddy", not(feature = "hybrid")))]
        return "buddy";
        #[cfg(all(feature = "bitmap", not(feature = "buddy"), not(feature = "hybrid")))]
        return "bitmap";
        #[cfg(not(any(feature = "buddy", feature = "bitmap", feature = "hybrid")))]
        return "unknown";
    }
    name
}

fn run_basic_only(allocator_name: &str, config: &TestConfig) {
    let name = allocator_name_or_default(allocator_name);
    
    #[cfg(feature = "buddy")]
    if name == "buddy" {
        let allocator = BuddyAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_basic_performance_test(&allocator, &config.basic_perf);
        std::println!("{}", metrics.to_report());
        return;
    }
    
    #[cfg(feature = "bitmap")]
    if name == "bitmap" {
        let allocator = BitmapAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_basic_performance_test(&allocator, &config.basic_perf);
        std::println!("{}", metrics.to_report());
        return;
    }
    
    #[cfg(feature = "hybrid")]
    if name == "hybrid" {
        let allocator = HybridAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_basic_performance_test(&allocator, &config.basic_perf);
        std::println!("{}", metrics.to_report());
        return;
    }
    
    std::println!("Allocator not available: {}", name);
}

fn run_fragmentation_only(allocator_name: &str, config: &TestConfig) {
    let name = allocator_name_or_default(allocator_name);
    
    #[cfg(feature = "buddy")]
    if name == "buddy" {
        let allocator = BuddyAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_fragmentation_test(&allocator, &config.fragmentation, config.memory_pool_size);
        std::println!("{}", metrics.to_report());
        return;
    }
    
    #[cfg(feature = "bitmap")]
    if name == "bitmap" {
        let allocator = BitmapAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_fragmentation_test(&allocator, &config.fragmentation, config.memory_pool_size);
        std::println!("{}", metrics.to_report());
        return;
    }
    
    #[cfg(feature = "hybrid")]
    if name == "hybrid" {
        let allocator = HybridAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_fragmentation_test(&allocator, &config.fragmentation, config.memory_pool_size);
        std::println!("{}", metrics.to_report());
        return;
    }
    
    std::println!("Allocator not available: {}", name);
}

fn run_stability_only(allocator_name: &str, config: &TestConfig) {
    let name = allocator_name_or_default(allocator_name);
    
    #[cfg(feature = "buddy")]
    if name == "buddy" {
        let allocator = BuddyAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_stability_test(&allocator, &config.stability);
        std::println!("{}", metrics.to_report());
        return;
    }
    
    #[cfg(feature = "bitmap")]
    if name == "bitmap" {
        let allocator = BitmapAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_stability_test(&allocator, &config.stability);
        std::println!("{}", metrics.to_report());
        return;
    }
    
    #[cfg(feature = "hybrid")]
    if name == "hybrid" {
        let allocator = HybridAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_stability_test(&allocator, &config.stability);
        std::println!("{}", metrics.to_report());
        return;
    }
    
    std::println!("Allocator not available: {}", name);
}

fn run_legacy_workload(workload: &str) {
    use allocator_tester::AllocatorTester;
    use workloads::{SmallObjectWorkload, LargeObjectWorkload, MixedWorkload, Workload};
    
    #[cfg(feature = "buddy")]
    let allocator: Box<dyn PageAllocator> = Box::new(BuddyAllocator::new());
    #[cfg(all(feature = "bitmap", not(feature = "buddy")))]
    let allocator: Box<dyn PageAllocator> = Box::new(BitmapAllocator::new());
    #[cfg(all(feature = "hybrid", not(feature = "buddy"), not(feature = "bitmap")))]
    let allocator: Box<dyn PageAllocator> = Box::new(HybridAllocator::new());
    
    #[cfg(any(feature = "buddy", feature = "bitmap", feature = "hybrid"))]
    {
        allocator.init(0x1000, 0x1000000).unwrap(); // 16MB
        
        match workload {
            "small" => {
                let tc = SmallObjectWorkload.generate_test_case();
                let result = AllocatorTester::run_test(&*allocator, &tc);
                std::println!("Small Object Workload Result: {:?}", result);
            }
            "large" => {
                let tc = LargeObjectWorkload.generate_test_case();
                let result = AllocatorTester::run_test(&*allocator, &tc);
                std::println!("Large Object Workload Result: {:?}", result);
            }
            "mixed" | _ => {
                let tc = MixedWorkload.generate_test_case();
                let result = AllocatorTester::run_test(&*allocator, &tc);
                std::println!("Mixed Workload Result: {:?}", result);
            }
        }
    }
}

fn print_help() {
    std::println!("Usage: allocator_test [OPTIONS] [WORKLOAD]");
    std::println!();
    std::println!("Workloads:");
    std::println!("  all, complete    Run complete test suite (default)");
    std::println!("  basic            Run basic performance tests only");
    std::println!("  fragmentation    Run fragmentation tests only");
    std::println!("  stability        Run stability tests only");
    std::println!("  legacy, small, large, mixed  Run legacy workloads");
    std::println!();
    std::println!("Options:");
    std::println!("  -a, --allocator NAME   Select allocator (buddy, bitmap, hybrid)");
    std::println!("  -c, --config LEVEL     Config level (small, medium, large)");
    std::println!("  --compare              Compare all available allocators");
    std::println!("  -h, --help             Show this help");
    std::println!();
    std::println!("Examples:");
    std::println!("  allocator_test all");
    std::println!("  allocator_test --allocator buddy --config large basic");
    std::println!("  allocator_test --compare");
}

/// Legacy entry point.
pub fn run_allocator_tests() {
    run_default_allocator_tests();
}