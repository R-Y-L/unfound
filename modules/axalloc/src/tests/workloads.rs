//! Workloads: Define various test workloads for allocators.
//!
//! This module provides predefined workloads for testing allocators,
//! including small object, large object, and mixed workloads.

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
            allocation_sizes: vec![1; 100], // 100 allocations of 1 page
            allocation_order: (0..100).collect(),
            deallocation_order: (0..100).rev().collect(),
        }
    }
}

/// Large object workload: Allocates fewer large objects.
pub struct LargeObjectWorkload;

impl Workload for LargeObjectWorkload {
    fn generate_test_case(&self) -> AllocatorTestCase {
        AllocatorTestCase {
            allocation_sizes: vec![64; 10], // 10 allocations of 64 pages
            allocation_order: (0..10).collect(),
            deallocation_order: (0..10).rev().collect(),
        }
    }
}

/// Mixed workload: Allocates a mix of small and large objects.
pub struct MixedWorkload;

impl Workload for MixedWorkload {
    fn generate_test_case(&self) -> AllocatorTestCase {
        AllocatorTestCase {
            allocation_sizes: vec![1, 64, 1, 64, 1, 64],
            allocation_order: vec![0, 1, 2, 3, 4, 5],
            deallocation_order: vec![5, 4, 3, 2, 1, 0],
        }
    }
}