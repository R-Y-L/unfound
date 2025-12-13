//! Test module: Entry point for allocator tests.
//!
//! This module integrates the allocator tester and workloads to run tests
//! and output results.

mod allocator_tester;
mod workloads;

use crate::allocators::BuddyAllocator;
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
    let buddy_allocator = BuddyAllocator::new();
    buddy_allocator.init(0x1000, 0x10000).unwrap();

    match workload.as_str() {
        "all" => run_all_tests(&buddy_allocator),
        "small" => run_single_test(&buddy_allocator, "Small Object Workload", SmallObjectWorkload),
        "large" => run_single_test(&buddy_allocator, "Large Object Workload", LargeObjectWorkload),
        "mixed" => run_single_test(&buddy_allocator, "Mixed Workload", MixedWorkload),
        _ => {
            println!("Unknown workload: {}", workload);
            println!("Available workloads: all, small, large, mixed");
        }
    }
}

/// Run all allocator tests.
pub fn run_allocator_tests() {
    let buddy_allocator = BuddyAllocator::new();
    buddy_allocator.init(0x1000, 0x10000).unwrap();
    run_all_tests(&buddy_allocator);
}

fn run_all_tests(allocator: &BuddyAllocator) {
    println!("Running Small Object Workload...");
    run_single_test(allocator, "Small Object Workload", SmallObjectWorkload);

    println!("Running Large Object Workload...");
    run_single_test(allocator, "Large Object Workload", LargeObjectWorkload);

    println!("Running Mixed Workload...");
    run_single_test(allocator, "Mixed Workload", MixedWorkload);
}

fn run_single_test<W: workloads::Workload>(allocator: &BuddyAllocator, name: &str, workload: W) {
    let result = AllocatorTester::run_test(allocator, &workload.generate_test_case());
    println!("{} Result: {:?}", name, result);
}