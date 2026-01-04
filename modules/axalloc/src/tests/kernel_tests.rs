//! Kernel-level allocator testing framework.
//!
//! This module provides allocator testing capabilities for no_std environments.
//! It can be used within the ArceOS kernel or similar embedded systems.
//!
//! Unlike the std-based test framework, this module:
//! - Uses no heap allocation during testing (or minimal allocation)
//! - Works with interrupt-disabled contexts
//! - Provides simpler timing using architecture-specific counters

#![allow(dead_code)]
#![allow(unused_imports)]

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::allocators::PageAllocator;

const PAGE_SIZE: usize = 4096;

// =============================================================================
// Kernel-compatible timing (uses simple counter when no std available)
// =============================================================================

/// Simple cycle counter for timing in no_std environments.
#[cfg(target_arch = "x86_64")]
fn rdtsc() -> u64 {
    unsafe {
        core::arch::x86_64::_rdtsc()
    }
}

#[cfg(target_arch = "aarch64")]
fn rdtsc() -> u64 {
    let cnt: u64;
    unsafe {
        core::arch::asm!("mrs {}, cntvct_el0", out(reg) cnt);
    }
    cnt
}

#[cfg(target_arch = "riscv64")]
fn rdtsc() -> u64 {
    let cnt: u64;
    unsafe {
        core::arch::asm!("rdtime {}", out(reg) cnt);
    }
    cnt
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "riscv64")))]
fn rdtsc() -> u64 {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed) as u64
}

// =============================================================================
// Simple RNG for deterministic testing
// =============================================================================

/// Simple Linear Congruential Generator.
pub struct KernelRng {
    state: u64,
}

impl KernelRng {
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }

    pub fn next_usize(&mut self, max: usize) -> usize {
        (self.next_u64() as usize) % max.max(1)
    }

    pub fn next_f32(&mut self) -> f32 {
        (self.next_u64() as f32) / (u64::MAX as f32)
    }
}

// =============================================================================
// Kernel Test Results
// =============================================================================

/// Basic performance results for kernel testing.
#[derive(Debug, Clone, Default)]
pub struct KernelTestResult {
    /// Allocator name.
    pub allocator_name: &'static str,
    /// Total allocation attempts.
    pub total_allocations: usize,
    /// Successful allocations.
    pub successful_allocations: usize,
    /// Failed allocations.
    pub failed_allocations: usize,
    /// Success rate (0-100).
    pub success_rate_percent: u32,
    /// Average allocation cycles.
    pub avg_alloc_cycles: u64,
    /// Average deallocation cycles.
    pub avg_dealloc_cycles: u64,
    /// External fragmentation rate (0-100).
    pub external_frag_percent: u32,
    /// Internal fragmentation rate (0-100).
    pub internal_frag_percent: u32,
    /// Memory leak detected.
    pub leak_detected: bool,
    /// Leaked bytes.
    pub leaked_bytes: usize,
}

impl KernelTestResult {
    pub fn new(name: &'static str) -> Self {
        Self {
            allocator_name: name,
            ..Default::default()
        }
    }

    /// Log the result using the kernel log macros.
    pub fn log_result(&self) {
        log::info!("========== {} Allocator Test Results ==========", self.allocator_name);
        log::info!("Allocations: {}/{} ({:.1}% success rate)",
            self.successful_allocations,
            self.total_allocations,
            self.success_rate_percent as f32
        );
        log::info!("Avg alloc cycles: {}, Avg dealloc cycles: {}",
            self.avg_alloc_cycles,
            self.avg_dealloc_cycles
        );
        log::info!("External fragmentation: {}%, Internal fragmentation: {}%",
            self.external_frag_percent,
            self.internal_frag_percent
        );
        if self.leak_detected {
            log::warn!("Memory leak detected: {} bytes", self.leaked_bytes);
        } else {
            log::info!("No memory leaks detected");
        }
        log::info!("================================================");
    }
}

/// Time-dimension test results for kernel.
#[derive(Debug, Clone, Default)]
pub struct KernelTimeDimensionResult {
    pub allocator_name: &'static str,
    /// Measurements at different simulated hours.
    pub measurements: Vec<KernelMeasurement>,
}

#[derive(Debug, Clone, Default)]
pub struct KernelMeasurement {
    pub simulated_hours: u32,
    pub success_rate_percent: u32,
    pub avg_alloc_cycles: u64,
    pub external_frag_percent: u32,
    pub failure_rate_16kb_percent: u32,
    pub failure_rate_1mb_percent: u32,
}

impl KernelTimeDimensionResult {
    pub fn log_result(&self) {
        log::info!("========== {} Time-Dimension Results ==========", self.allocator_name);
        for m in &self.measurements {
            log::info!("{}h: success={}%, frag={}%, 16KB fail={}%, 1MB fail={}%",
                m.simulated_hours,
                m.success_rate_percent,
                m.external_frag_percent,
                m.failure_rate_16kb_percent,
                m.failure_rate_1mb_percent
            );
        }
        log::info!("================================================");
    }
}

// =============================================================================
// Kernel Test Functions
// =============================================================================

/// Run basic performance test on an allocator.
/// 
/// This is designed to work in kernel context with minimal overhead.
pub fn kernel_basic_test<A: PageAllocator + ?Sized>(
    allocator: &A,
    num_allocations: usize,
) -> KernelTestResult {
    let mut result = KernelTestResult::new(allocator.name());
    result.total_allocations = num_allocations;
    
    let mut rng = KernelRng::new(12345);
    let mut alloc_cycles_total: u64 = 0;
    let mut dealloc_cycles_total: u64 = 0;
    let mut allocated: Vec<(usize, usize)> = Vec::with_capacity(num_allocations);
    
    let sizes = [1, 4, 16, 64];
    
    // Allocation phase
    for _ in 0..num_allocations {
        let size = sizes[rng.next_usize(sizes.len())];
        
        let start = rdtsc();
        let res = allocator.alloc_pages(size, PAGE_SIZE);
        let elapsed = rdtsc().wrapping_sub(start);
        
        match res {
            Ok(addr) => {
                result.successful_allocations += 1;
                alloc_cycles_total += elapsed;
                allocated.push((addr, size));
            }
            Err(_) => {
                result.failed_allocations += 1;
            }
        }
    }
    
    // Deallocation phase
    for (addr, size) in allocated.iter() {
        let start = rdtsc();
        allocator.dealloc_pages(*addr, *size);
        let elapsed = rdtsc().wrapping_sub(start);
        dealloc_cycles_total += elapsed;
    }
    
    // Calculate metrics
    if result.successful_allocations > 0 {
        result.avg_alloc_cycles = alloc_cycles_total / result.successful_allocations as u64;
        result.avg_dealloc_cycles = dealloc_cycles_total / result.successful_allocations as u64;
    }
    
    result.success_rate_percent = if result.total_allocations > 0 {
        ((result.successful_allocations * 100) / result.total_allocations) as u32
    } else {
        0
    };
    
    // Get fragmentation from allocator stats
    let (frag_rate, _total_free) = allocator.get_stats();
    result.external_frag_percent = (frag_rate * 100.0) as u32;
    
    result
}

/// Run fragmentation test.
pub fn kernel_fragmentation_test<A: PageAllocator + ?Sized>(
    allocator: &A,
    num_operations: usize,
) -> KernelTestResult {
    let mut result = KernelTestResult::new(allocator.name());
    let mut rng = KernelRng::new(67890);
    
    let mut allocated: Vec<(usize, usize, usize)> = Vec::new(); // (addr, actual, requested)
    let mut total_allocated = 0usize;
    let mut total_wasted = 0usize;
    
    let sizes = [1, 4, 16, 64, 256];
    
    // Mixed allocate/deallocate operations
    for _ in 0..num_operations {
        let should_dealloc = !allocated.is_empty() && rng.next_f32() < 0.6;
        
        if should_dealloc {
            let idx = rng.next_usize(allocated.len());
            let (addr, actual, requested) = allocated.swap_remove(idx);
            allocator.dealloc_pages(addr, requested);
            total_allocated -= actual;
            total_wasted -= actual - (requested * PAGE_SIZE);
        } else {
            let size = sizes[rng.next_usize(sizes.len())];
            result.total_allocations += 1;
            
            if let Ok(addr) = allocator.alloc_pages(size, PAGE_SIZE) {
                result.successful_allocations += 1;
                let actual = size.next_power_of_two() * PAGE_SIZE;
                let requested_bytes = size * PAGE_SIZE;
                let waste = actual - requested_bytes;
                
                allocated.push((addr, actual, size));
                total_allocated += actual;
                total_wasted += waste;
            } else {
                result.failed_allocations += 1;
            }
        }
    }
    
    // Calculate fragmentation
    let (frag_rate, _) = allocator.get_stats();
    result.external_frag_percent = (frag_rate * 100.0) as u32;
    
    if total_allocated > 0 {
        result.internal_frag_percent = ((total_wasted * 100) / total_allocated) as u32;
    }
    
    result.success_rate_percent = if result.total_allocations > 0 {
        ((result.successful_allocations * 100) / result.total_allocations) as u32
    } else {
        0
    };
    
    // Clean up
    for (addr, _, size) in allocated {
        allocator.dealloc_pages(addr, size);
    }
    
    result
}

/// Run leak detection test.
pub fn kernel_leak_test<A: PageAllocator + ?Sized>(
    allocator: &A,
    iterations: usize,
) -> KernelTestResult {
    let mut result = KernelTestResult::new(allocator.name());
    let mut rng = KernelRng::new(11111);
    
    let initial_stats = allocator.get_stats();
    let initial_free = initial_stats.1;
    
    // Allocate and immediately free many times
    for _ in 0..iterations {
        let size = 1 + rng.next_usize(16);
        if let Ok(addr) = allocator.alloc_pages(size, PAGE_SIZE) {
            allocator.dealloc_pages(addr, size);
            result.successful_allocations += 1;
        }
        result.total_allocations += 1;
    }
    
    let final_stats = allocator.get_stats();
    let final_free = final_stats.1;
    
    if initial_free > final_free + PAGE_SIZE {
        result.leak_detected = true;
        result.leaked_bytes = initial_free - final_free;
    }
    
    result.success_rate_percent = if result.total_allocations > 0 {
        ((result.successful_allocations * 100) / result.total_allocations) as u32
    } else {
        0
    };
    
    result
}

/// Run simulated time-dimension test.
pub fn kernel_time_dimension_test<A: PageAllocator + ?Sized>(
    allocator: &A,
    ops_per_hour: usize,
    hours: &[u32],
) -> KernelTimeDimensionResult {
    let mut result = KernelTimeDimensionResult {
        allocator_name: allocator.name(),
        measurements: Vec::new(),
    };
    
    let mut rng = KernelRng::new(54321);
    let mut allocated: Vec<(usize, usize)> = Vec::new();
    
    let sizes = [1, 4, 16, 64, 256];
    let max_hour = hours.iter().max().copied().unwrap_or(1);
    
    // Initial measurement
    if hours.contains(&0) {
        result.measurements.push(KernelMeasurement {
            simulated_hours: 0,
            success_rate_percent: 100,
            avg_alloc_cycles: 0,
            external_frag_percent: 0,
            failure_rate_16kb_percent: 0,
            failure_rate_1mb_percent: 0,
        });
    }
    
    for hour in 1..=max_hour {
        // Simulate operations for this hour
        let mut successes = 0;
        let mut attempts = 0;
        let mut total_cycles: u64 = 0;
        
        for _ in 0..ops_per_hour {
            let should_dealloc = !allocated.is_empty() && rng.next_f32() < 0.6;
            
            if should_dealloc {
                let idx = rng.next_usize(allocated.len());
                let (addr, size) = allocated.swap_remove(idx);
                allocator.dealloc_pages(addr, size);
            } else {
                let size = sizes[rng.next_usize(sizes.len())];
                attempts += 1;
                
                let start = rdtsc();
                if let Ok(addr) = allocator.alloc_pages(size, PAGE_SIZE) {
                    total_cycles += rdtsc().wrapping_sub(start);
                    successes += 1;
                    allocated.push((addr, size));
                }
            }
        }
        
        // Take measurement if this is a target hour
        if hours.contains(&hour) {
            let (frag_rate, _) = allocator.get_stats();
            
            // Test failure rates for specific sizes
            let fr_16kb = measure_failure_rate(allocator, 4, 50);
            let fr_1mb = measure_failure_rate(allocator, 256, 50);
            
            result.measurements.push(KernelMeasurement {
                simulated_hours: hour,
                success_rate_percent: if attempts > 0 { ((successes * 100) / attempts) as u32 } else { 0 },
                avg_alloc_cycles: if successes > 0 { total_cycles / successes as u64 } else { 0 },
                external_frag_percent: (frag_rate * 100.0) as u32,
                failure_rate_16kb_percent: fr_16kb,
                failure_rate_1mb_percent: fr_1mb,
            });
        }
    }
    
    // Clean up
    for (addr, size) in allocated {
        allocator.dealloc_pages(addr, size);
    }
    
    result
}

fn measure_failure_rate<A: PageAllocator + ?Sized>(allocator: &A, size_pages: usize, attempts: usize) -> u32 {
    let mut failures = 0;
    for _ in 0..attempts {
        match allocator.alloc_pages(size_pages, PAGE_SIZE) {
            Ok(addr) => allocator.dealloc_pages(addr, size_pages),
            Err(_) => failures += 1,
        }
    }
    ((failures * 100) / attempts) as u32
}

/// Run all kernel tests and log results.
pub fn run_all_kernel_tests<A: PageAllocator + ?Sized>(allocator: &A) {
    log::info!("Starting kernel allocator tests for {}...", allocator.name());
    
    // Basic test
    let basic = kernel_basic_test(allocator, 1000);
    basic.log_result();
    
    // Fragmentation test
    let frag = kernel_fragmentation_test(allocator, 5000);
    frag.log_result();
    
    // Leak test
    let leak = kernel_leak_test(allocator, 10000);
    leak.log_result();
    
    // Time dimension test
    let td = kernel_time_dimension_test(allocator, 1000, &[0, 1, 6, 24]);
    td.log_result();
    
    log::info!("Kernel allocator tests complete.");
}
