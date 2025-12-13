//! Binary for running allocator tests from command line.

use std::collections::BTreeMap;
use std::vec::Vec;
use std::cmp;
use kspin::SpinNoIrq;
use memory_addr::is_aligned;
use allocator::AllocError;
use axalloc::allocators::PageAllocator;

// Copy of BuddyAllocator for testing (since it's not public)
const PAGE_SIZE: usize = 4096;

pub struct BuddyAllocator {
    base: usize,
    total_pages: usize,
    max_order: usize,
    free_lists: SpinNoIrq<Vec<Vec<usize>>>,
    alloc_map: SpinNoIrq<BTreeMap<usize, usize>>,
    used_pages: SpinNoIrq<usize>,
}

impl BuddyAllocator {
    pub fn new() -> Self {
        Self {
            base: 0,
            total_pages: 0,
            max_order: 0,
            free_lists: SpinNoIrq::new(Vec::new()),
            alloc_map: SpinNoIrq::new(BTreeMap::new()),
            used_pages: SpinNoIrq::new(0),
        }
    }

    fn push_free(&self, order: usize, idx: usize) {
        let mut lists = self.free_lists.lock();
        if order >= lists.len() {
            lists.resize(order + 1, Vec::new());
        }
        lists[order].push(idx);
    }

    fn pop_free(&self, order: usize) -> Option<usize> {
        let mut lists = self.free_lists.lock();
        if order >= lists.len() { return None; }
        lists[order].pop()
    }

    fn remove_free_exact(&self, order: usize, idx: usize) -> bool {
        let mut lists = self.free_lists.lock();
        if order >= lists.len() { return false; }
        if let Some(pos) = lists[order].iter().position(|&x| x == idx) {
            lists[order].swap_remove(pos);
            true
        } else { false }
    }

    fn free_list_snapshot(&self) -> Vec<Vec<usize>> {
        self.free_lists.lock().clone()
    }

    fn used_pages(&self) -> usize {
        *self.used_pages.lock()
    }
}

impl PageAllocator for BuddyAllocator {
    fn name(&self) -> &'static str { "buddy" }

    fn init(&self, start_vaddr: usize, size: usize) -> Result<(), AllocError> {
        let end = (start_vaddr + size) & !(PAGE_SIZE - 1);
        let start = (start_vaddr + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        if end <= start { return Err(AllocError::InvalidParam); }
        let total_pages = (end - start) / PAGE_SIZE;
        if total_pages == 0 { return Err(AllocError::InvalidParam); }

        let mut mo = 0usize;
        while (1usize << (mo + 1)) <= total_pages { mo += 1; }

        {
            let mut lists = self.free_lists.lock();
            lists.clear();
            lists.resize(mo + 1, Vec::new());
        }
        self.alloc_map.lock().clear();
        *self.used_pages.lock() = 0;

        let mut remaining = total_pages;
        let mut offset = 0usize;
        while remaining > 0 {
            let order = (usize::BITS as usize - 1) - (remaining.leading_zeros() as usize);
            let block_size = 1usize << order;
            self.push_free(order, offset);
            offset += block_size;
            remaining -= block_size;
        }

        unsafe {
            let s = self as *const Self as *mut Self;
            (*s).base = start;
            (*s).total_pages = total_pages;
            (*s).max_order = mo;
        }

        Ok(())
    }

    fn alloc_pages(&self, num_pages: usize, align_pow2: usize) -> Result<usize, AllocError> {
        if num_pages == 0 { return Err(AllocError::InvalidParam); }
        if align_pow2 < PAGE_SIZE || !align_pow2.is_power_of_two() { return Err(AllocError::InvalidParam); }

        let needed = num_pages.next_power_of_two();
        let order = (needed as f64).log2() as usize;
        let mut o = order;
        while o <= self.max_order {
            if let Some(idx) = self.pop_free(o) {
                let mut cur_idx = idx;
                let mut cur_order = o;
                while cur_order > order {
                    cur_order -= 1;
                    let buddy_idx = cur_idx + (1usize << cur_order);
                    self.push_free(cur_order, buddy_idx);
                }
                self.alloc_map.lock().insert(cur_idx, order);
                *self.used_pages.lock() += 1usize << order;
                return Ok(self.base + cur_idx * PAGE_SIZE);
            }
            o += 1;
        }
        Err(AllocError::NoMemory)
    }

    fn alloc_pages_at(&self, start: usize, num_pages: usize, align_pow2: usize) -> Result<usize, AllocError> {
        if num_pages == 0 { return Err(AllocError::InvalidParam); }
        if align_pow2 < PAGE_SIZE || !align_pow2.is_power_of_two() { return Err(AllocError::InvalidParam); }
        if start < self.base || start >= self.base + self.total_pages * PAGE_SIZE { return Err(AllocError::InvalidParam); }
        if !is_aligned(start, align_pow2) { return Err(AllocError::InvalidParam); }
        let idx = (start - self.base) / PAGE_SIZE;
        let needed = num_pages.next_power_of_two();
        let order = (needed as f64).log2() as usize;
        if self.remove_free_exact(order, idx) {
            self.alloc_map.lock().insert(idx, order);
            *self.used_pages.lock() += 1usize << order;
            return Ok(start);
        }
        Err(AllocError::NoMemory)
    }

    fn dealloc_pages(&self, pos: usize, _num_pages: usize) {
        if pos < self.base || pos >= self.base + self.total_pages * PAGE_SIZE { return; }
        if !is_aligned(pos, PAGE_SIZE) { return; }
        let mut idx = (pos - self.base) / PAGE_SIZE;
        let order = match self.alloc_map.lock().remove(&idx) {
            Some(o) => o,
            None => return,
        };
        let mut cur_order = order;
        loop {
            let buddy_idx = idx ^ (1usize << cur_order);
            if self.remove_free_exact(cur_order, buddy_idx) {
                idx = cmp::min(idx, buddy_idx);
                cur_order += 1;
                if cur_order > self.max_order { break; }
                continue;
            } else { break; }
        }
        self.push_free(cur_order, idx);
        *self.used_pages.lock() -= 1usize << order;
    }
}

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
    pub fn run_test(
        allocator: &BuddyAllocator,
        test_case: &AllocatorTestCase,
    ) -> TestResult {
        let mut total_alloc_time = 0u64;
        let mut total_dealloc_time = 0u64;
        let mut successful_allocations = 0;
        let mut failed_allocations = 0;
        let mut peak_memory_usage = 0;

        let start_alloc = std::time::Instant::now();
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

        let start_dealloc = std::time::Instant::now();
        for &size in &test_case.deallocation_order {
            allocator.dealloc_pages(size, 1);
        }
        total_dealloc_time = start_dealloc.elapsed().as_nanos() as u64;

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

fn run_all_tests(allocator: &BuddyAllocator) {
    println!("Running Small Object Workload...");
    run_single_test(allocator, "Small Object Workload", SmallObjectWorkload);

    println!("Running Large Object Workload...");
    run_single_test(allocator, "Large Object Workload", LargeObjectWorkload);

    println!("Running Mixed Workload...");
    run_single_test(allocator, "Mixed Workload", MixedWorkload);
}

fn run_single_test<W: Workload>(allocator: &BuddyAllocator, name: &str, workload: W) {
    let result = AllocatorTester::run_test(allocator, &workload.generate_test_case());
    println!("{} Result: {:?}", name, result);
}