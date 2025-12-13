//! Allocator Tester: Core testing framework for memory allocators.
//!
//! This module provides the `AllocatorTester` struct to test memory allocators
//! and collect performance metrics.

use std::time::Instant;
use crate::allocators::PageAllocator;

/// Test result structure to store metrics.
#[derive(Debug)]
pub struct TestResult {
    pub total_allocations: usize,
    pub successful_allocations: usize,
    pub failed_allocations: usize,
    pub average_allocation_time_ns: u64,
    pub average_deallocation_time_ns: u64,
    pub fragmentation: f64,
    pub peak_memory_usage: usize,
    pub remaining_free_memory: usize,
}

/// Test case structure to define allocation and deallocation patterns.
pub struct AllocatorTestCase {
    pub allocation_sizes: Vec<usize>,
    pub allocation_order: Vec<usize>,
    pub deallocation_order: Vec<usize>,
}

/// Allocator Tester: Executes test cases and collects metrics.
pub struct AllocatorTester;

impl AllocatorTester {
    /// Run a test case on the given allocator.
    pub fn run_test<A: PageAllocator>(
        allocator: &A,
        test_case: &AllocatorTestCase,
    ) -> TestResult {
        let mut total_alloc_time = 0u64;
        let mut total_dealloc_time = 0u64;
        let mut successful_allocations = 0;
        let mut failed_allocations = 0;
        let mut peak_memory_usage = 0;

        // Allocation phase
        let start_alloc = Instant::now();
        for &size in &test_case.allocation_sizes {
            let result = allocator.alloc_pages(size, 4096);
            if result.is_ok() {
                successful_allocations += 1;
                peak_memory_usage += size * 4096;
            } else {
                failed_allocations += 1;
            }
        }
        total_alloc_time = start_alloc.elapsed().as_nanos() as u64;

        // Deallocation phase
        let start_dealloc = Instant::now();
        for &size in &test_case.deallocation_order {
            allocator.dealloc_pages(size, 1); // Assume deallocating 1 page
        }
        total_dealloc_time = start_dealloc.elapsed().as_nanos() as u64;

        // Calculate fragmentation
        let free_list = allocator.free_list_snapshot();
        let largest_free_block = free_list.iter().flatten().max().unwrap_or(&0);
        let total_free_memory: usize = free_list.iter().flatten().sum();
        let fragmentation = 1.0 - (*largest_free_block as f64 / total_free_memory as f64);

        TestResult {
            total_allocations: test_case.allocation_sizes.len(),
            successful_allocations,
            failed_allocations,
            average_allocation_time_ns: total_alloc_time / test_case.allocation_sizes.len() as u64,
            average_deallocation_time_ns: total_dealloc_time / test_case.deallocation_order.len() as u64,
            fragmentation,
            peak_memory_usage,
            remaining_free_memory: total_free_memory,
        }
    }
}