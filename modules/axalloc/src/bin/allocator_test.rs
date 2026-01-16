//! Binary for running allocator tests from command line.
//!
//! This is the main entry point for the allocator testing framework.
//! It supports comprehensive testing of memory allocators with various
//! test suites and configurations.
//!
//! ## Usage
//!
//! ```bash
//! # Run complete test suite
//! cargo run --bin allocator_test --features "buddy std" all
//!
//! # Run specific test suite
//! cargo run --bin allocator_test --features "buddy std" basic
//! cargo run --bin allocator_test --features "buddy std" fragmentation
//! cargo run --bin allocator_test --features "buddy std" stability
//! cargo run --bin allocator_test --features "buddy std" time-dimension
//!
//! # Compare allocators
//! cargo run --bin allocator_test --features "buddy bitmap hybrid std" --compare
//!
//! # Specify allocator and config
//! cargo run --bin allocator_test --features "buddy std" -a buddy -c large all
//! ```

use axalloc::tests::{
    run_allocator_tests_from_cli,
    run_tests_by_name,
    compare_allocators,
    print_comparison_table,
    run_basic_performance_test,
    run_fragmentation_test,
    run_stability_test,
    run_time_dimension_test,
    TestConfig,
    TimeDimensionConfig,
};
use axalloc::allocators::PageAllocator;

#[cfg(feature = "buddy")]
use axalloc::allocators::BuddyAllocator;
#[cfg(feature = "bitmap")]
use axalloc::allocators::BitmapAllocator;
#[cfg(feature = "hybrid")]
use axalloc::allocators::HybridAllocator;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }
    
    // Parse arguments
    let mut workload = "all".to_string();
    let mut allocator_name = "default".to_string();
    let mut config_level = "medium".to_string();
    let mut compare_mode = false;
    let mut verbose = false;
    
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
            "--verbose" | "-v" => {
                verbose = true;
            }
            "--help" | "-h" => {
                print_usage(&args[0]);
                return;
            }
            other => {
                // First positional arg is workload
                if !other.starts_with('-') && i == 1 {
                    workload = other.to_string();
                } else if !other.starts_with('-') {
                    workload = other.to_string();
                }
            }
        }
        i += 1;
    }
    
    // Select config
    let mut config = match config_level.as_str() {
        "small" | "quick" => TestConfig::small(),
        "medium" | "default" => TestConfig::medium(),
        "large" | "stress" => TestConfig::large(),
        _ => TestConfig::medium(),
    };
    config.verbose = verbose;
    
    if compare_mode {
        run_comparison(&config);
    } else {
        run_workload(&workload, &allocator_name, &config);
    }
}

fn print_usage(prog: &str) {
    println!("Allocator Testing Framework");
    println!("============================");
    println!();
    println!("Usage: {} [OPTIONS] [WORKLOAD]", prog);
    println!();
    println!("Workloads:");
    println!("  all, complete        Run complete test suite (default)");
    println!("  basic                Run basic performance tests");
    println!("  fragmentation        Run fragmentation tests");
    println!("  stability            Run stability tests");
    println!("  time-dimension       Run time-dimension (long-running) tests");
    println!("  legacy               Run legacy workload tests");
    println!();
    println!("Options:");
    println!("  -a, --allocator NAME   Select allocator (buddy, bitmap, hybrid)");
    println!("  -c, --config LEVEL     Config level (small, medium, large)");
    println!("  --compare              Compare all available allocators");
    println!("  -v, --verbose          Enable verbose output");
    println!("  -h, --help             Show this help");
    println!();
    println!("Examples:");
    println!("  {} all", prog);
    println!("  {} --allocator buddy basic", prog);
    println!("  {} --config large time-dimension", prog);
    println!("  {} --compare", prog);
    println!();
    println!("Compile with features:");
    println!("  --features \"buddy std\"      Enable buddy allocator");
    println!("  --features \"bitmap std\"     Enable bitmap allocator");
    println!("  --features \"hybrid std\"     Enable hybrid allocator");
    println!("  --features \"buddy bitmap hybrid std\"  Enable all for comparison");
}

fn get_default_allocator_name() -> &'static str {
    // Priority: buddy > bitmap > hybrid
    #[cfg(feature = "buddy")]
    return "buddy";
    #[cfg(all(feature = "bitmap", not(feature = "buddy")))]
    return "bitmap";
    #[cfg(all(feature = "hybrid", not(feature = "buddy"), not(feature = "bitmap")))]
    return "hybrid";
    #[cfg(not(any(feature = "buddy", feature = "bitmap", feature = "hybrid")))]
    return "none";
}

fn run_comparison(config: &TestConfig) {
    println!("Running allocator comparison...\n");
    
    let names: Vec<&str> = vec![
        #[cfg(feature = "buddy")]
        "buddy",
        #[cfg(feature = "bitmap")]
        "bitmap",
        #[cfg(feature = "hybrid")]
        "hybrid",
    ];
    
    if names.is_empty() {
        println!("No allocators available. Enable at least one allocator feature.");
        return;
    }
    
    let results = compare_allocators(&names, config);
    print_comparison_table(&results);
    
    println!("\nDetailed Reports:\n");
    for (_name, report) in &results {
        println!("{}", report.to_full_report());
    }
}

fn run_workload(workload: &str, allocator_name: &str, config: &TestConfig) {
    let name = if allocator_name == "default" {
        get_default_allocator_name()
    } else {
        allocator_name
    };
    
    if name == "none" {
        println!("No allocator available. Enable an allocator feature.");
        return;
    }
    
    match workload {
        "all" | "complete" => {
            println!("Running complete test suite for {}...\n", name);
            match run_tests_by_name(name, config) {
                Ok(report) => println!("{}", report.to_full_report()),
                Err(e) => println!("Error: {}", e),
            }
        }
        "basic" => {
            run_basic_test(name, config);
        }
        "fragmentation" => {
            run_frag_test(name, config);
        }
        "stability" => {
            run_stab_test(name, config);
        }
        "time-dimension" | "timedim" | "td" => {
            run_timedim_test(name, config);
        }
        "legacy" => {
            run_allocator_tests_from_cli();
        }
        _ => {
            println!("Unknown workload: {}", workload);
            print_usage("allocator_test");
        }
    }
}

fn run_basic_test(name: &str, config: &TestConfig) {
    println!("Running basic performance tests for {}...\n", name);
    
    #[cfg(feature = "buddy")]
    if name == "buddy" {
        let allocator = BuddyAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_basic_performance_test(&allocator, &config.basic_perf);
        println!("{}", metrics.to_report());
        return;
    }
    
    #[cfg(feature = "bitmap")]
    if name == "bitmap" {
        let allocator = BitmapAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_basic_performance_test(&allocator, &config.basic_perf);
        println!("{}", metrics.to_report());
        return;
    }
    
    #[cfg(feature = "hybrid")]
    if name == "hybrid" {
        let allocator = HybridAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_basic_performance_test(&allocator, &config.basic_perf);
        println!("{}", metrics.to_report());
        return;
    }
    
    println!("Allocator not available: {}", name);
}

fn run_frag_test(name: &str, config: &TestConfig) {
    println!("Running fragmentation tests for {}...\n", name);
    
    #[cfg(feature = "buddy")]
    if name == "buddy" {
        let allocator = BuddyAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_fragmentation_test(&allocator, &config.fragmentation, config.memory_pool_size);
        println!("{}", metrics.to_report());
        return;
    }
    
    #[cfg(feature = "bitmap")]
    if name == "bitmap" {
        let allocator = BitmapAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_fragmentation_test(&allocator, &config.fragmentation, config.memory_pool_size);
        println!("{}", metrics.to_report());
        return;
    }
    
    #[cfg(feature = "hybrid")]
    if name == "hybrid" {
        let allocator = HybridAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_fragmentation_test(&allocator, &config.fragmentation, config.memory_pool_size);
        println!("{}", metrics.to_report());
        return;
    }
    
    println!("Allocator not available: {}", name);
}

fn run_stab_test(name: &str, config: &TestConfig) {
    println!("Running stability tests for {}...\n", name);
    
    #[cfg(feature = "buddy")]
    if name == "buddy" {
        let allocator = BuddyAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_stability_test(&allocator, &config.stability);
        println!("{}", metrics.to_report());
        return;
    }
    
    #[cfg(feature = "bitmap")]
    if name == "bitmap" {
        let allocator = BitmapAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_stability_test(&allocator, &config.stability);
        println!("{}", metrics.to_report());
        return;
    }
    
    #[cfg(feature = "hybrid")]
    if name == "hybrid" {
        let allocator = HybridAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let metrics = run_stability_test(&allocator, &config.stability);
        println!("{}", metrics.to_report());
        return;
    }
    
    println!("Allocator not available: {}", name);
}

fn run_timedim_test(name: &str, config: &TestConfig) {
    println!("Running time-dimension tests for {}...\n", name);
    println!("This simulates 24 hours of allocator operation.\n");
    
    let td_config = if config.memory_pool_size < 128 * 1024 * 1024 {
        TimeDimensionConfig::quick()
    } else {
        TimeDimensionConfig::default()
    };
    
    #[cfg(feature = "buddy")]
    if name == "buddy" {
        let allocator = BuddyAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let results = run_time_dimension_test(&allocator, &td_config);
        println!("{}", results.to_report());
        return;
    }
    
    #[cfg(feature = "bitmap")]
    if name == "bitmap" {
        let allocator = BitmapAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let results = run_time_dimension_test(&allocator, &td_config);
        println!("{}", results.to_report());
        return;
    }
    
    #[cfg(feature = "hybrid")]
    if name == "hybrid" {
        let allocator = HybridAllocator::new();
        allocator.init(config.start_vaddr, config.memory_pool_size).unwrap();
        let results = run_time_dimension_test(&allocator, &td_config);
        println!("{}", results.to_report());
        return;
    }
    
    println!("Allocator not available: {}", name);
}