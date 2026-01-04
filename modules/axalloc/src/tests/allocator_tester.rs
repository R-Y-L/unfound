//! Allocator Tester: Core testing framework for memory allocators.
//!
//! This module provides the `AllocatorTester` struct to test memory allocators
//! and collect performance metrics.

extern crate std;
extern crate alloc;

use std::time::Instant;
use alloc::vec::Vec;
use alloc::vec;
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
    pub fn run_test<A: PageAllocator + ?Sized>(
        allocator: &A,
        test_case: &AllocatorTestCase,
    ) -> TestResult {
        let mut successful_allocations = 0;
        let mut failed_allocations = 0;
        let mut peak_memory_usage = 0;

        // Record allocated addresses so we can free by address later
        let mut allocated_addrs: Vec<Option<usize>> = vec![None; test_case.allocation_sizes.len()];

        // Allocation phase
        let start_alloc = Instant::now();
        for (i, &size) in test_case.allocation_sizes.iter().enumerate() {
            match allocator.alloc_pages(size, 4096) {
                Ok(addr) => {
                    successful_allocations += 1;
                    allocated_addrs[i] = Some(addr);
                    peak_memory_usage += size * 4096;
                }
                Err(_) => {
                    failed_allocations += 1;
                }
            }
        }
        let total_alloc_time = start_alloc.elapsed().as_nanos() as u64;

        // Deallocation phase - use actual addresses
        let start_dealloc = Instant::now();
        for &idx in &test_case.deallocation_order {
            if let Some(Some(addr)) = allocated_addrs.get(idx) {
                allocator.dealloc_pages(*addr, 1);
            }
        }
        let total_dealloc_time = start_dealloc.elapsed().as_nanos() as u64;

        // Get fragmentation and free memory from allocator's diagnostic stats
        let (fragmentation, total_free_memory) = allocator.get_stats();

        TestResult {
            total_allocations: test_case.allocation_sizes.len(),
            successful_allocations,
            failed_allocations,
            average_allocation_time_ns: if test_case.allocation_sizes.len() > 0 {
                total_alloc_time / test_case.allocation_sizes.len() as u64
            } else {
                0
            },
            average_deallocation_time_ns: if test_case.deallocation_order.len() > 0 {
                total_dealloc_time / test_case.deallocation_order.len() as u64
            } else {
                0
            },
            fragmentation,
            peak_memory_usage,
            remaining_free_memory: total_free_memory,
        }
    }
}