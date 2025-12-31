//! Test module: Entry point for allocator tests.
//!
//! This module integrates the allocator tester and workloads to run tests
//! and output results.

mod allocator_tester;
mod workloads;

use crate::allocators::PageAllocator;

#[cfg(feature = "buddy")]
use crate::allocators::BuddyAllocator;
#[cfg(feature = "bitmap")]
use crate::allocators::BitmapAllocator;
#[cfg(feature = "hybrid")]
use crate::allocators::HybridAllocator;

use allocator_tester::AllocatorTester;
use workloads::{SmallObjectWorkload, LargeObjectWorkload, MixedWorkload};

/// Run allocator tests from command line arguments.
/// Usage: cargo run --bin allocator_test <workload>
/// Workloads: all, small, large, mixed
pub fn run_allocator_tests_from_cli() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <workload>", args[0]);
        println!("Workloads: all, small, large, mixed");
        return;
    }

    let workload = &args[1];

    // Create allocator based on compile-time feature
    #[cfg(feature = "buddy")]
    let allocator: Box<dyn PageAllocator> = Box::new(BuddyAllocator::new());
    #[cfg(all(feature = "bitmap", not(feature = "buddy")))]
    let allocator: Box<dyn PageAllocator> = Box::new(BitmapAllocator::new());
    #[cfg(all(feature = "hybrid", not(feature = "buddy"), not(feature = "bitmap")))]
    let allocator: Box<dyn PageAllocator> = Box::new(HybridAllocator::new());

    // Initialize with larger memory pool: 16MB instead of 64KB for better success rate
    allocator.init(0x1000, 0x1000000).unwrap(); // 16MB

    match workload.as_str() {
        "all" => run_all_tests(&*allocator),
        "small" => run_single_test(&*allocator, "Small Object Workload", SmallObjectWorkload),
        "large" => run_single_test(&*allocator, "Large Object Workload", LargeObjectWorkload),
        "mixed" => run_single_test(&*allocator, "Mixed Workload", MixedWorkload),
        _ => {
            println!("Unknown workload: {}", workload);
            println!("Available workloads: all, small, large, mixed");
        }
    }
}

/// Run all allocator tests.
pub fn run_allocator_tests() {
    #[cfg(feature = "buddy")]
    let allocator: Box<dyn PageAllocator> = Box::new(BuddyAllocator::new());
    #[cfg(all(feature = "bitmap", not(feature = "buddy")))]
    let allocator: Box<dyn PageAllocator> = Box::new(BitmapAllocator::new());
    #[cfg(all(feature = "hybrid", not(feature = "buddy"), not(feature = "bitmap")))]
    let allocator: Box<dyn PageAllocator> = Box::new(HybridAllocator::new());

    // Initialize with larger memory pool: 16MB
    allocator.init(0x1000, 0x1000000).unwrap();
    run_all_tests(&*allocator);
}

fn run_all_tests(allocator: &dyn PageAllocator) {
    println!("Running Small Object Workload...");
    run_single_test(allocator, "Small Object Workload", SmallObjectWorkload);

    println!("Running Large Object Workload...");
    run_single_test(allocator, "Large Object Workload", LargeObjectWorkload);

    println!("Running Mixed Workload...");
    run_single_test(allocator, "Mixed Workload", MixedWorkload);
}

fn run_single_test<W: workloads::Workload>(allocator: &dyn PageAllocator, name: &str, workload: W) {
    let result = AllocatorTester::run_test(allocator, &workload.generate_test_case());
    println!("{} Result: {:?}", name, result);
}