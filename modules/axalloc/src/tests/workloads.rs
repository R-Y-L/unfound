//! Workloads: Define various test workloads for allocators.
//!
//! This module provides predefined workloads for testing allocators,
//! including small object, large object, and mixed workloads.

extern crate alloc;

use alloc::vec;
use crate::tests::allocator_tester::AllocatorTestCase;

/// Workload trait: Defines a common interface for all workloads.
pub trait Workload {
    fn generate_test_case(&self) -> AllocatorTestCase;
}

/// Small object workload: Allocates many small objects.
pub struct SmallObjectWorkload;

impl Workload for SmallObjectWorkload {
    fn generate_test_case(&self) -> AllocatorTestCase {
        AllocatorTestCase {
            allocation_sizes: vec![1; 500], // 500 allocations of 1 page (increased from 100)
            allocation_order: (0..500).collect(),
            deallocation_order: (0..500).rev().collect(),
        }
    }
}

/// Large object workload: Allocates fewer large objects.
pub struct LargeObjectWorkload;

impl Workload for LargeObjectWorkload {
    fn generate_test_case(&self) -> AllocatorTestCase {
        AllocatorTestCase {
            allocation_sizes: vec![64; 50], // 50 allocations of 64 pages (increased from 10)
            allocation_order: (0..50).collect(),
            deallocation_order: (0..50).rev().collect(),
        }
    }
}

/// Mixed workload: Allocates a mix of small and large objects.
pub struct MixedWorkload;

impl Workload for MixedWorkload {
    fn generate_test_case(&self) -> AllocatorTestCase {
        AllocatorTestCase {
            allocation_sizes: vec![1, 16, 4, 32, 1, 8, 2, 64, 1, 4, 16, 2], // Mixed sizes, more realistic
            allocation_order: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            deallocation_order: vec![11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        }
    }
}