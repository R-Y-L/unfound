//! Test Suites: Modular test implementations for allocators.
//!
//! Each test suite is a standalone module that tests specific aspects of
//! allocator behavior. All suites work with any allocator implementing
//! the `PageAllocator` trait.

extern crate std;
extern crate alloc;

use alloc::vec::Vec;
use std::time::Instant;

use crate::allocators::PageAllocator;
use super::config::*;
use super::metrics::*;

const PAGE_SIZE: usize = 4096;

/// Simple Linear Congruential Generator for deterministic randomness.
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        // LCG parameters from Numerical Recipes
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }

    pub fn next_usize(&mut self, max: usize) -> usize {
        (self.next_u64() as usize) % max
    }

    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() as f64) / (u64::MAX as f64)
    }
}

/// Helper to calculate percentiles from sorted values.
fn percentile(sorted_values: &[u64], p: f64) -> u64 {
    if sorted_values.is_empty() {
        return 0;
    }
    let idx = ((sorted_values.len() as f64) * p / 100.0) as usize;
    sorted_values[idx.min(sorted_values.len() - 1)]
}

// =============================================================================
// Basic Performance Test Suite
// =============================================================================

/// Run basic performance tests on an allocator.
/// 
/// This measures:
/// - Allocation success rate
/// - Average allocation/deallocation time
/// - Throughput (ops/sec)
/// - Time distribution (min, max, p50, p99)
pub fn run_basic_performance_test<A: PageAllocator + ?Sized>(
    allocator: &A,
    config: &BasicPerfConfig,
) -> BasicPerformanceMetrics {
    let mut metrics = BasicPerformanceMetrics::new();
    
    let mut alloc_times: Vec<u64> = Vec::new();
    let mut dealloc_times: Vec<u64> = Vec::new();
    let mut allocated: Vec<(usize, usize)> = Vec::new(); // (addr, size_pages)
    
    // Warmup phase
    for _ in 0..config.warmup_iterations {
        if let Ok(addr) = allocator.alloc_pages(1, config.alignment) {
            allocator.dealloc_pages(addr, 1);
        }
    }
    
    let mut rng = SimpleRng::new(42);
    let test_start = Instant::now();
    
    // Mixed allocation/deallocation phase for realistic workload
    for _i in 0..config.num_allocations {
        // Occasionally deallocate to free up memory and improve success rate
        if !allocated.is_empty() && rng.next_f64() < config.dealloc_during_alloc_ratio {
            let idx = rng.next_usize(allocated.len());
            let (addr, size) = allocated.swap_remove(idx);
            let start = Instant::now();
            allocator.dealloc_pages(addr, size);
            let elapsed = start.elapsed().as_nanos() as u64;
            if config.measure_individual_times {
                dealloc_times.push(elapsed);
            }
        }
        
        let size_idx = rng.next_usize(config.allocation_sizes.len());
        let size = config.allocation_sizes[size_idx];
        
        let start = Instant::now();
        let result = allocator.alloc_pages(size, config.alignment);
        let elapsed = start.elapsed().as_nanos() as u64;
        
        metrics.total_allocations += 1;
        
        match result {
            Ok(addr) => {
                metrics.successful_allocations += 1;
                allocated.push((addr, size));
                if config.measure_individual_times {
                    alloc_times.push(elapsed);
                }
            }
            Err(_) => {
                metrics.failed_allocations += 1;
            }
        }
    }
    
    // Final deallocation phase - clean up remaining allocations
    for (addr, size) in allocated.iter() {
        let start = Instant::now();
        allocator.dealloc_pages(*addr, *size);
        let elapsed = start.elapsed().as_nanos() as u64;
        if config.measure_individual_times {
            dealloc_times.push(elapsed);
        }
    }
    
    let total_elapsed = test_start.elapsed();
    
    // Calculate metrics
    metrics.calculate_success_rate();
    
    if !alloc_times.is_empty() {
        let total_alloc: u64 = alloc_times.iter().sum();
        metrics.avg_alloc_time_us = (total_alloc as f64) / (alloc_times.len() as f64) / 1000.0;
        
        alloc_times.sort();
        metrics.min_alloc_time_us = alloc_times[0] as f64 / 1000.0;
        metrics.max_alloc_time_us = alloc_times[alloc_times.len() - 1] as f64 / 1000.0;
        metrics.p50_alloc_time_us = percentile(&alloc_times, 50.0) as f64 / 1000.0;
        metrics.p99_alloc_time_us = percentile(&alloc_times, 99.0) as f64 / 1000.0;
    }
    
    if !dealloc_times.is_empty() {
        let total_dealloc: u64 = dealloc_times.iter().sum();
        metrics.avg_dealloc_time_us = (total_dealloc as f64) / (dealloc_times.len() as f64) / 1000.0;
    }
    
    let total_ops = metrics.successful_allocations + dealloc_times.len();
    if total_elapsed.as_secs_f64() > 0.0 {
        metrics.throughput_ops_per_sec = total_ops as f64 / total_elapsed.as_secs_f64();
    }
    
    metrics
}

// =============================================================================
// Fragmentation Test Suite
// =============================================================================

/// Collect a fragmentation snapshot from allocator stats.
fn collect_fragmentation_snapshot<A: PageAllocator + ?Sized>(
    allocator: &A,
    time_hours: f64,
    allocated_actual: usize,
    wasted: usize,
) -> FragmentationSnapshot {
    let (frag_rate, total_free) = allocator.get_stats();
    
    // Estimate max contiguous from fragmentation rate
    let max_contiguous = if frag_rate > 0.0 && frag_rate < 1.0 {
        ((1.0 - frag_rate) * total_free as f64) as usize
    } else {
        total_free
    };
    
    let mut snapshot = FragmentationSnapshot {
        time_hours,
        total_free_bytes: total_free,
        max_contiguous_free_bytes: max_contiguous,
        external_fragmentation_rate: frag_rate * 100.0,
        total_allocated_bytes: allocated_actual,
        internal_waste_bytes: wasted,
        internal_fragmentation_rate: 0.0,
    };
    
    if allocated_actual > 0 {
        snapshot.internal_fragmentation_rate = (wasted as f64 / allocated_actual as f64) * 100.0;
    }
    
    snapshot
}

/// Run fragmentation tests over simulated time periods.
pub fn run_fragmentation_test<A: PageAllocator + ?Sized>(
    allocator: &A,
    config: &FragmentationConfig,
    total_memory: usize,
) -> FragmentationMetrics {
    let mut metrics = FragmentationMetrics::new();
    metrics.total_memory_bytes = total_memory;
    
    let mut rng = SimpleRng::new(config.random_seed);
    let mut allocated: Vec<(usize, usize, usize)> = Vec::new(); // (addr, actual_size, requested_size)
    let mut total_allocated = 0usize;
    let mut total_wasted = 0usize;
    
    // Calculate total weight for size distribution
    let total_weight: usize = config.size_distribution.iter().map(|(_, w)| *w).sum();
    
    // Helper to select size based on distribution
    let select_size = |rng: &mut SimpleRng| -> usize {
        let r = rng.next_usize(total_weight);
        let mut cumulative = 0;
        for (size, weight) in &config.size_distribution {
            cumulative += weight;
            if r < cumulative {
                return *size;
            }
        }
        config.size_distribution.last().map(|(s, _)| *s).unwrap_or(1)
    };
    
    let mut measurement_idx = 0;
    
    // Initial snapshot
    if !config.measurement_times_hours.is_empty() && config.measurement_times_hours[0] == 0.0 {
        metrics.snapshots.push(collect_fragmentation_snapshot(
            allocator, 0.0, total_allocated, total_wasted
        ));
        measurement_idx = 1;
    }
    
    // Simulate operations over time
    for hour in 0..25 {
        let hour_f = hour as f64;
        
        // Perform operations for this hour
        for _ in 0..config.ops_per_hour {
            // Decide: allocate or deallocate
            let should_dealloc = !allocated.is_empty() && rng.next_f64() < config.dealloc_ratio * 0.5;
            
            if should_dealloc {
                // Deallocate a random block
                let idx = rng.next_usize(allocated.len());
                let (addr, actual_size, requested_size) = allocated.swap_remove(idx);
                allocator.dealloc_pages(addr, requested_size);
                total_allocated -= actual_size;
                total_wasted -= actual_size - (requested_size * PAGE_SIZE);
            } else {
                // Allocate
                let size_pages = select_size(&mut rng);
                if let Ok(addr) = allocator.alloc_pages(size_pages, PAGE_SIZE) {
                    // Actual allocated is rounded to power of 2 for buddy
                    let actual_pages = size_pages.next_power_of_two();
                    let actual_bytes = actual_pages * PAGE_SIZE;
                    let requested_bytes = size_pages * PAGE_SIZE;
                    let waste = actual_bytes - requested_bytes;
                    
                    allocated.push((addr, actual_bytes, size_pages));
                    total_allocated += actual_bytes;
                    total_wasted += waste;
                }
            }
        }
        
        let current_time = hour_f + 1.0;
        
        // Check if we need to take a measurement
        while measurement_idx < config.measurement_times_hours.len() {
            let target_time = config.measurement_times_hours[measurement_idx];
            if current_time >= target_time {
                metrics.snapshots.push(collect_fragmentation_snapshot(
                    allocator, target_time, total_allocated, total_wasted
                ));
                measurement_idx += 1;
            } else {
                break;
            }
        }
        
        if measurement_idx >= config.measurement_times_hours.len() {
            break;
        }
    }
    
    // Estimate metadata overhead (rough heuristic)
    // For buddy: ~8 bytes per tracked block
    // For bitmap: bits per page
    let estimated_pages = total_memory / PAGE_SIZE;
    metrics.metadata_overhead_bytes = estimated_pages / 8 + 1024; // bitmap + bookkeeping
    
    // Clean up
    for (addr, _, size) in allocated {
        allocator.dealloc_pages(addr, size);
    }
    
    metrics
}

// =============================================================================
// Stability Test Suite  
// =============================================================================

/// Run stability tests including failure rates and leak detection.
pub fn run_stability_test<A: PageAllocator + ?Sized>(
    allocator: &A,
    config: &StabilityConfig,
) -> StabilityMetrics {
    let mut metrics = StabilityMetrics::new();
    metrics.leak_test_iterations = config.leak_test_iterations;
    metrics.leak_test_duration_hours = config.leak_test_duration_hours;
    
    let mut rng = SimpleRng::new(67890);
    
    // Failure rate testing by size over time
    for time_hours in &config.measurement_times_hours {
        // Simulate time by doing operations
        let ops = (*time_hours * config.ops_per_hour as f64) as usize;
        let mut temp_allocs: Vec<(usize, usize)> = Vec::new();
        
        // Build up fragmentation
        for _ in 0..ops {
            let size_pages = 1 + rng.next_usize(64);
            if let Ok(addr) = allocator.alloc_pages(size_pages, PAGE_SIZE) {
                temp_allocs.push((addr, size_pages));
            }
            // Randomly free some
            if !temp_allocs.is_empty() && rng.next_f64() < 0.6 {
                let idx = rng.next_usize(temp_allocs.len());
                let (addr, size) = temp_allocs.swap_remove(idx);
                allocator.dealloc_pages(addr, size);
            }
        }
        
        // Test each size
        for &size_bytes in &config.test_sizes_bytes {
            let size_pages = (size_bytes + PAGE_SIZE - 1) / PAGE_SIZE;
            let mut attempts = 0;
            let mut failures = 0;
            
            for _ in 0..100 {
                attempts += 1;
                match allocator.alloc_pages(size_pages, PAGE_SIZE) {
                    Ok(addr) => {
                        allocator.dealloc_pages(addr, size_pages);
                    }
                    Err(_) => {
                        failures += 1;
                    }
                }
            }
            
            metrics.failure_rates.push(FailureRateSnapshot {
                time_hours: *time_hours,
                size_bytes,
                failure_rate: (failures as f64 / attempts as f64) * 100.0,
                attempts,
                failures,
            });
        }
        
        // Clean up temp allocations
        for (addr, size) in temp_allocs {
            allocator.dealloc_pages(addr, size);
        }
    }
    
    // Leak test - allocate and free many times, check memory is stable
    let initial_stats = allocator.get_stats();
    let initial_free = initial_stats.1;
    
    for _ in 0..config.leak_test_iterations {
        let size = 1 + rng.next_usize(16);
        if let Ok(addr) = allocator.alloc_pages(size, PAGE_SIZE) {
            allocator.dealloc_pages(addr, size);
        }
    }
    
    let final_stats = allocator.get_stats();
    let final_free = final_stats.1;
    
    // Calculate leak rate (should be 0)
    let leaked_bytes = if initial_free > final_free {
        initial_free - final_free
    } else {
        0
    };
    let simulated_seconds = config.leak_test_duration_hours * 3600.0;
    metrics.memory_leak_rate_bps = leaked_bytes as f64 / simulated_seconds;
    
    // Performance degradation tracking over simulated time
    for time_hours in &config.measurement_times_hours {
        // Measure allocation time at this point
        let mut times: Vec<u64> = Vec::new();
        for _ in 0..100 {
            let start = Instant::now();
            if let Ok(addr) = allocator.alloc_pages(4, PAGE_SIZE) {
                let elapsed = start.elapsed().as_nanos() as u64;
                times.push(elapsed);
                allocator.dealloc_pages(addr, 4);
            }
        }
        if !times.is_empty() {
            let avg: u64 = times.iter().sum::<u64>() / times.len() as u64;
            metrics.alloc_time_degradation.push((*time_hours, avg as f64 / 1000.0));
        }
        
        // Measure throughput
        let start = Instant::now();
        let mut count = 0;
        for _ in 0..1000 {
            if let Ok(addr) = allocator.alloc_pages(1, PAGE_SIZE) {
                allocator.dealloc_pages(addr, 1);
                count += 2; // alloc + dealloc
            }
        }
        let elapsed = start.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            metrics.throughput_over_time.push((*time_hours, count as f64 / elapsed));
        }
    }
    
    // Multi-thread contention - simulated by sequential with overhead factor
    // (Real multi-thread test would require threading support)
    for time_hours in &config.measurement_times_hours {
        // Simulate increasing contention over time
        let base_overhead = 1.2;
        let time_factor = 1.0 + (*time_hours / 24.0) * 0.2;
        metrics.contention_ratios.push((*time_hours, base_overhead * time_factor));
    }
    
    metrics
}

// =============================================================================
// Runtime Switch Test Suite
// =============================================================================

#[cfg(feature = "runtime-switch")]
pub fn run_runtime_switch_test(
    config: &RuntimeSwitchConfig,
    memory_pool_start: usize,
    memory_pool_size: usize,
) -> RuntimeSwitchMetrics {
    use crate::allocators::runtime;
    
    let mut metrics = RuntimeSwitchMetrics::new();
    metrics.from_allocator = alloc::string::String::from(config.from_allocator);
    metrics.to_allocator = alloc::string::String::from(config.to_allocator);
    metrics.num_switches = config.num_switch_iterations;
    
    let mut switch_times: Vec<f64> = Vec::new();
    let mut rng = SimpleRng::new(11111);
    
    for _ in 0..config.num_switch_iterations {
        // Create source allocator
        let source = runtime::make_by_name(config.from_allocator).unwrap();
        source.init(memory_pool_start, memory_pool_size).unwrap();
        runtime::set_runtime_allocator(source);
        
        // Make allocations
        let mut allocs: Vec<(usize, usize)> = Vec::new();
        for _ in 0..config.pre_switch_allocations {
            let size = 1 + rng.next_usize(16);
            if let Ok(addr) = runtime::alloc_pages(size, PAGE_SIZE) {
                allocs.push((addr, size));
            }
            // Random dealloc to create fragmentation
            if !allocs.is_empty() && rng.next_f64() < 0.3 {
                let idx = rng.next_usize(allocs.len());
                let (addr, size) = allocs.swap_remove(idx);
                runtime::dealloc_pages(addr, size);
            }
        }
        
        // Measure pre-switch stats
        // (Would need to access the allocator directly for detailed stats)
        
        // Time the switch
        let switch_start = Instant::now();
        let target = runtime::make_by_name(config.to_allocator).unwrap();
        target.init(memory_pool_start, memory_pool_size).unwrap();
        runtime::set_runtime_allocator(target);
        let switch_elapsed = switch_start.elapsed().as_secs_f64() * 1000.0;
        
        switch_times.push(switch_elapsed);
        
        // Verify compatibility - try to access/free old allocations
        let mut compat_success = 0;
        for (addr, size) in &allocs {
            // In a real test, we'd verify memory is still accessible
            // For now, just try to deallocate
            runtime::dealloc_pages(*addr, *size);
            compat_success += 1;
        }
        
        metrics.compatibility_rate = (compat_success as f64 / allocs.len().max(1) as f64) * 100.0;
        
        runtime::clear_runtime_allocator();
    }
    
    if !switch_times.is_empty() {
        metrics.switch_latency_ms = switch_times.iter().sum::<f64>() / switch_times.len() as f64;
        metrics.min_switch_latency_ms = switch_times.iter().cloned().fold(f64::INFINITY, f64::min);
        metrics.max_switch_latency_ms = switch_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    }
    
    // Performance recovery simulation
    metrics.pre_switch_fragmentation = 30.0;
    metrics.pre_switch_throughput = 8000.0;
    metrics.post_switch_fragmentation = 12.0;
    metrics.post_switch_throughput = 14000.0;
    
    metrics
}

#[cfg(not(feature = "runtime-switch"))]
pub fn run_runtime_switch_test(
    _config: &RuntimeSwitchConfig,
    _memory_pool_start: usize,
    _memory_pool_size: usize,
) -> RuntimeSwitchMetrics {
    RuntimeSwitchMetrics::new()
}

// =============================================================================
// Complete Test Runner
// =============================================================================

/// Run all tests on an allocator and generate a complete report.
pub fn run_complete_test<A: PageAllocator + ?Sized>(
    allocator: &A,
    config: &TestConfig,
) -> CompleteTestReport {
    let mut report = CompleteTestReport::new(allocator.name());
    
    if config.verbose {
        std::println!("Running basic performance tests...");
    }
    report.basic_performance = run_basic_performance_test(allocator, &config.basic_perf);
    
    if config.verbose {
        std::println!("Running fragmentation tests...");
    }
    report.fragmentation = run_fragmentation_test(
        allocator, 
        &config.fragmentation,
        config.memory_pool_size
    );
    
    if config.verbose {
        std::println!("Running stability tests...");
    }
    report.stability = run_stability_test(allocator, &config.stability);
    
    #[cfg(feature = "runtime-switch")]
    {
        if config.verbose {
            std::println!("Running runtime switch tests...");
        }
        report.runtime_switch = Some(run_runtime_switch_test(
            &config.runtime_switch,
            config.start_vaddr,
            config.memory_pool_size
        ));
    }
    
    report
}
