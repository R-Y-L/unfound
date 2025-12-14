//! Binary for running allocator tests from command line.

use axalloc::allocators::PageAllocator;

// Import concrete allocators based on feature flags and expose unified testing
#[cfg(feature = "buddy")]
use axalloc::allocators::BuddyAllocator;
#[cfg(feature = "bitmap")]
use axalloc::allocators::BitmapAllocator;
#[cfg(all(feature = "hybrid", not(feature = "buddy"), not(feature = "bitmap")))]
use axalloc::allocators::HybridAllocator;

const PAGE_SIZE: usize = 4096;

// Simplified test structures
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

pub struct AllocatorTestCase {
    pub allocation_sizes: Vec<usize>,
    pub allocation_order: Vec<usize>,
    pub deallocation_order: Vec<usize>,
}

pub struct AllocatorTester;

impl AllocatorTester {
    pub fn run(
        allocator: &dyn PageAllocator,
        test_case: &AllocatorTestCase,
    ) -> TestResult {
        // Track timings and counts
        let mut successful_allocations = 0usize;
        let mut failed_allocations = 0usize;
        let mut peak_memory_usage = 0usize;

        // Record allocated addresses so we can free by address later
        let mut allocated_addrs: Vec<Option<usize>> = vec![None; test_case.allocation_sizes.len()];

        let start_alloc = std::time::Instant::now();
        for (i, &size) in test_case.allocation_sizes.iter().enumerate() {
            match allocator.alloc_pages(size, PAGE_SIZE) {
                Ok(addr) => {
                    successful_allocations += 1;
                    allocated_addrs[i] = Some(addr);
                    peak_memory_usage += size * PAGE_SIZE;
                }
                Err(_) => {
                    failed_allocations += 1;
                }
            }
        }
        let total_alloc_time = start_alloc.elapsed().as_nanos() as u64;

        // Deallocate by using recorded addresses according to deallocation_order
        let start_dealloc = std::time::Instant::now();
        for &idx in &test_case.deallocation_order {
            if let Some(Some(addr)) = allocated_addrs.get(idx).cloned() {
                allocator.dealloc_pages(addr, 1);
            }
        }
        let total_dealloc_time = start_dealloc.elapsed().as_nanos() as u64;

        // Get fragmentation and free memory from allocator's diagnostic stats
        let (fragmentation, total_free_memory) = allocator.get_stats();

        TestResult {
            total_allocations: test_case.allocation_sizes.len(),
            successful_allocations,
            failed_allocations,
            average_allocation_time_ns: if test_case.allocation_sizes.len() > 0 { total_alloc_time / test_case.allocation_sizes.len() as u64 } else { 0 },
            average_deallocation_time_ns: if test_case.deallocation_order.len() > 0 { total_dealloc_time / test_case.deallocation_order.len() as u64 } else { 0 },
            fragmentation,
            peak_memory_usage,
            remaining_free_memory: total_free_memory,
        }
    }
}

impl AllocatorTester {
    /// Backwards-compatible name used by callers in this binary.
    pub fn run_test(
        allocator: &dyn PageAllocator,
        test_case: &AllocatorTestCase,
    ) -> TestResult {
        Self::run(allocator, test_case)
    }
}

// Workloads
pub trait Workload {
    fn generate_test_case(&self) -> AllocatorTestCase;
}

pub struct SmallObjectWorkload;

impl Workload for SmallObjectWorkload {
    fn generate_test_case(&self) -> AllocatorTestCase {
        AllocatorTestCase {
            allocation_sizes: vec![1; 100],
            allocation_order: (0..100).collect(),
            deallocation_order: (0..100).rev().collect(),
        }
    }
}

pub struct LargeObjectWorkload;

impl Workload for LargeObjectWorkload {
    fn generate_test_case(&self) -> AllocatorTestCase {
        AllocatorTestCase {
            allocation_sizes: vec![64; 10],
            allocation_order: (0..10).collect(),
            deallocation_order: (0..10).rev().collect(),
        }
    }
}

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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <workload>", args[0]);
        println!("Workloads: all, small, large, mixed");
        return;
    }

    let workload = &args[1];

    // Instantiate the allocator selected at compile time (one feature should be enabled)
    #[cfg(feature = "buddy")]
    let allocator: Box<dyn PageAllocator> = Box::new(BuddyAllocator::new());
    #[cfg(all(feature = "bitmap", not(feature = "buddy")))]
    let allocator: Box<dyn PageAllocator> = Box::new(BitmapAllocator::new());
    #[cfg(all(feature = "hybrid", not(feature = "buddy"), not(feature = "bitmap")))]
    let allocator: Box<dyn PageAllocator> = Box::new(HybridAllocator::new());

    // Initialize allocator with a modest test region (increase if you want large-object tests)
    allocator.init(0x1000, 0x10000).unwrap();

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

fn run_all_tests(allocator: &dyn PageAllocator) {
    println!("Running Small Object Workload...");
    run_single_test(allocator, "Small Object Workload", SmallObjectWorkload);

    println!("Running Large Object Workload...");
    run_single_test(allocator, "Large Object Workload", LargeObjectWorkload);

    println!("Running Mixed Workload...");
    run_single_test(allocator, "Mixed Workload", MixedWorkload);
}

fn run_single_test<W: Workload>(allocator: &dyn PageAllocator, name: &str, workload: W) {
    let result = AllocatorTester::run_test(allocator, &workload.generate_test_case());
    println!("{} Result: {:?}", name, result);
}