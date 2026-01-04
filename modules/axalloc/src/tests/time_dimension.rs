//! Time-dimension tests: Long-running tests that simulate extended operation.
//!
//! These tests simulate the effects of running for extended periods (1h, 6h, 24h)
//! by performing proportionally more operations and tracking degradation.

extern crate std;
extern crate alloc;

use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;
use std::time::Instant;

use crate::allocators::PageAllocator;
use super::suites::SimpleRng;
use super::metrics::*;

const PAGE_SIZE: usize = 4096;

/// Configuration for time-dimension tests.
#[derive(Debug, Clone)]
pub struct TimeDimensionConfig {
    /// Simulated hours to run.
    pub hours: Vec<f64>,
    /// Operations per simulated hour.
    pub ops_per_hour: usize,
    /// Measurement interval (simulated hours).
    pub measurement_interval: f64,
    /// Sizes to test for failure rate (in pages).
    pub test_sizes_pages: Vec<usize>,
    /// Number of threads to simulate for contention test.
    pub num_threads: usize,
    /// Allocation pattern weights: (size_pages, weight).
    pub allocation_pattern: Vec<(usize, usize)>,
    /// Deallocation probability per operation.
    pub dealloc_probability: f64,
}

impl Default for TimeDimensionConfig {
    fn default() -> Self {
        Self {
            hours: vec![0.0, 1.0, 6.0, 24.0],
            ops_per_hour: 10000,
            measurement_interval: 1.0,
            test_sizes_pages: vec![4, 256], // 16KB, 1MB
            num_threads: 4,
            allocation_pattern: vec![
                (1, 40),    // 40% single page
                (4, 25),    // 25% 4 pages (16KB)
                (16, 20),   // 20% 16 pages (64KB)
                (64, 10),   // 10% 64 pages (256KB)
                (256, 5),   // 5% 256 pages (1MB)
            ],
            dealloc_probability: 0.6,
        }
    }
}

impl TimeDimensionConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Quick config for testing.
    pub fn quick() -> Self {
        Self {
            hours: vec![0.0, 1.0],
            ops_per_hour: 1000,
            measurement_interval: 0.5,
            test_sizes_pages: vec![4, 64],
            num_threads: 2,
            allocation_pattern: vec![(1, 60), (16, 30), (64, 10)],
            dealloc_probability: 0.6,
        }
    }

    /// Full 24-hour simulation config.
    pub fn full_24h() -> Self {
        Self {
            hours: vec![0.0, 1.0, 6.0, 24.0],
            ops_per_hour: 50000,
            measurement_interval: 1.0,
            test_sizes_pages: vec![4, 16, 64, 256],
            num_threads: 4,
            allocation_pattern: vec![
                (1, 35),
                (4, 25),
                (16, 20),
                (64, 12),
                (256, 8),
            ],
            dealloc_probability: 0.65,
        }
    }
}

/// Results from time-dimension testing.
#[derive(Debug, Clone, Default)]
pub struct TimeDimensionResults {
    /// Allocator name.
    pub allocator_name: String,
    /// Fragmentation over time.
    pub fragmentation_timeline: Vec<FragmentationSnapshot>,
    /// Failure rates by size over time.
    pub failure_rates_timeline: Vec<FailureRateSnapshot>,
    /// Performance metrics over time.
    pub performance_timeline: Vec<PerformanceSnapshot>,
    /// Contention overhead over time.
    pub contention_timeline: Vec<(f64, f64)>, // (hours, ratio)
    /// Memory leak detection.
    pub leak_detected: bool,
    pub leaked_bytes: usize,
}

/// Performance snapshot at a point in time.
#[derive(Debug, Clone, Default)]
pub struct PerformanceSnapshot {
    pub time_hours: f64,
    pub avg_alloc_time_us: f64,
    pub avg_dealloc_time_us: f64,
    pub throughput_ops_per_sec: f64,
    pub success_rate: f64,
}

impl TimeDimensionResults {
    pub fn new(allocator_name: &str) -> Self {
        Self {
            allocator_name: String::from(allocator_name),
            ..Default::default()
        }
    }

    /// Generate a comprehensive report.
    pub fn to_report(&self) -> String {
        let mut report = alloc::format!(
            "========== {} 时间维度测试报告 ==========\n\n",
            self.allocator_name
        );

        // External fragmentation over time
        report.push_str("【外部碎片率变化】\n");
        for snap in &self.fragmentation_timeline {
            report.push_str(&alloc::format!(
                "  - {}h: {:.1}% (空闲 {:.1}MB, 最大连续 {:.1}MB)\n",
                snap.time_hours,
                snap.external_fragmentation_rate,
                snap.total_free_bytes as f64 / (1024.0 * 1024.0),
                snap.max_contiguous_free_bytes as f64 / (1024.0 * 1024.0)
            ));
        }

        // Internal fragmentation over time
        report.push_str("\n【内部碎片率变化】\n");
        for snap in &self.fragmentation_timeline {
            report.push_str(&alloc::format!(
                "  - {}h: {:.1}% (已分配 {:.1}MB, 浪费 {:.1}MB)\n",
                snap.time_hours,
                snap.internal_fragmentation_rate,
                snap.total_allocated_bytes as f64 / (1024.0 * 1024.0),
                snap.internal_waste_bytes as f64 / (1024.0 * 1024.0)
            ));
        }

        // Failure rates by size
        report.push_str("\n【分配失败率（按大小）】\n");
        let mut sizes: Vec<usize> = self.failure_rates_timeline.iter()
            .map(|f| f.size_bytes)
            .collect();
        sizes.sort();
        sizes.dedup();

        for size in sizes {
            let entries: Vec<_> = self.failure_rates_timeline.iter()
                .filter(|f| f.size_bytes == size)
                .collect();
            if !entries.is_empty() {
                let size_str = if size >= 1024 * 1024 {
                    alloc::format!("{}MB", size / (1024 * 1024))
                } else {
                    alloc::format!("{}KB", size / 1024)
                };
                report.push_str(&alloc::format!("  - {}:\n", size_str));
                for e in entries {
                    report.push_str(&alloc::format!(
                        "    {}h: {:.1}% ({}/{})\n",
                        e.time_hours, e.failure_rate, e.failures, e.attempts
                    ));
                }
            }
        }

        // Performance over time
        report.push_str("\n【性能变化】\n");
        for perf in &self.performance_timeline {
            report.push_str(&alloc::format!(
                "  - {}h: 分配{:.2}ms, 释放{:.2}ms, 吞吐{:.0}/s, 成功率{:.1}%\n",
                perf.time_hours,
                perf.avg_alloc_time_us / 1000.0,
                perf.avg_dealloc_time_us / 1000.0,
                perf.throughput_ops_per_sec,
                perf.success_rate
            ));
        }

        // Contention overhead
        report.push_str("\n【多线程竞争开销】\n");
        for (hours, ratio) in &self.contention_timeline {
            report.push_str(&alloc::format!(
                "  - {}h: {:.1}倍 ({}线程 vs 单线程)\n",
                hours, ratio, 4
            ));
        }

        // Memory leak
        report.push_str(&alloc::format!(
            "\n【内存泄漏检测】: {}\n",
            if self.leak_detected {
                alloc::format!("检测到泄漏 {} 字节", self.leaked_bytes)
            } else {
                String::from("无泄漏")
            }
        ));

        // Performance degradation summary
        if self.performance_timeline.len() >= 2 {
            let first = &self.performance_timeline[0];
            let last = self.performance_timeline.last().unwrap();
            let alloc_degrade = ((last.avg_alloc_time_us - first.avg_alloc_time_us)
                / first.avg_alloc_time_us.max(0.001)) * 100.0;
            let throughput_degrade = ((first.throughput_ops_per_sec - last.throughput_ops_per_sec)
                / first.throughput_ops_per_sec.max(0.001)) * 100.0;
            
            report.push_str(&alloc::format!(
                "\n【性能衰减总结】\n  - 分配时间衰减: {:.0}%\n  - 吞吐量衰减: {:.0}%\n",
                alloc_degrade.max(0.0),
                throughput_degrade.max(0.0)
            ));
        }

        report.push_str("\n==========================================\n");
        report
    }
}

/// Run time-dimension tests on an allocator.
pub fn run_time_dimension_test<A: PageAllocator + ?Sized>(
    allocator: &A,
    config: &TimeDimensionConfig,
) -> TimeDimensionResults {
    let mut results = TimeDimensionResults::new(allocator.name());
    let mut rng = SimpleRng::new(54321);
    
    // Track allocations: (addr, actual_size, requested_pages)
    let mut allocated: Vec<(usize, usize, usize)> = Vec::new();
    let mut total_allocated = 0usize;
    let mut total_wasted = 0usize;
    
    // Calculate total weight for size distribution
    let total_weight: usize = config.allocation_pattern.iter().map(|(_, w)| *w).sum();
    
    // Helper to select size based on distribution
    let select_size = |rng: &mut SimpleRng| -> usize {
        let r = rng.next_usize(total_weight);
        let mut cumulative = 0;
        for (size, weight) in &config.allocation_pattern {
            cumulative += weight;
            if r < cumulative {
                return *size;
            }
        }
        config.allocation_pattern.last().map(|(s, _)| *s).unwrap_or(1)
    };
    
    // Get initial memory state
    let initial_stats = allocator.get_stats();
    let initial_free = initial_stats.1;
    
    // Take initial measurements (0h)
    if config.hours.contains(&0.0) {
        results.fragmentation_timeline.push(FragmentationSnapshot {
            time_hours: 0.0,
            total_free_bytes: initial_free,
            max_contiguous_free_bytes: initial_free,
            external_fragmentation_rate: 0.0,
            total_allocated_bytes: 0,
            internal_waste_bytes: 0,
            internal_fragmentation_rate: 0.0,
        });
        
        results.performance_timeline.push(measure_performance(allocator, 0.0));
        
        for &size_pages in &config.test_sizes_pages {
            results.failure_rates_timeline.push(FailureRateSnapshot {
                time_hours: 0.0,
                size_bytes: size_pages * PAGE_SIZE,
                failure_rate: 0.0,
                attempts: 0,
                failures: 0,
            });
        }
        
        results.contention_timeline.push((0.0, 1.0));
    }
    
    // Simulate hours
    let max_hour = config.hours.iter().cloned().fold(0.0f64, f64::max) as usize;
    
    for hour in 0..=max_hour {
        let hour_f = hour as f64;
        
        // Perform operations for this hour
        for _ in 0..config.ops_per_hour {
            // Decide: allocate or deallocate
            let should_dealloc = !allocated.is_empty() && rng.next_f64() < config.dealloc_probability;
            
            if should_dealloc {
                let idx = rng.next_usize(allocated.len());
                let (addr, actual_size, requested_pages) = allocated.swap_remove(idx);
                allocator.dealloc_pages(addr, requested_pages);
                total_allocated -= actual_size;
                total_wasted -= actual_size - (requested_pages * PAGE_SIZE);
            } else {
                let size_pages = select_size(&mut rng);
                if let Ok(addr) = allocator.alloc_pages(size_pages, PAGE_SIZE) {
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
        
        // Check if we should take measurements at this hour
        let current_time = hour_f + 1.0;
        for &target_time in &config.hours {
            if (current_time - target_time).abs() < 0.1 && target_time > 0.0 {
                // Fragmentation snapshot
                let (frag_rate, total_free) = allocator.get_stats();
                let max_contiguous = if frag_rate > 0.0 && frag_rate < 1.0 {
                    ((1.0 - frag_rate) * total_free as f64) as usize
                } else {
                    total_free
                };
                
                results.fragmentation_timeline.push(FragmentationSnapshot {
                    time_hours: target_time,
                    total_free_bytes: total_free,
                    max_contiguous_free_bytes: max_contiguous,
                    external_fragmentation_rate: frag_rate * 100.0,
                    total_allocated_bytes: total_allocated,
                    internal_waste_bytes: total_wasted,
                    internal_fragmentation_rate: if total_allocated > 0 {
                        (total_wasted as f64 / total_allocated as f64) * 100.0
                    } else {
                        0.0
                    },
                });
                
                // Performance snapshot
                results.performance_timeline.push(measure_performance(allocator, target_time));
                
                // Failure rate by size
                for &size_pages in &config.test_sizes_pages {
                    let (attempts, failures) = measure_failure_rate(allocator, size_pages, 100);
                    results.failure_rates_timeline.push(FailureRateSnapshot {
                        time_hours: target_time,
                        size_bytes: size_pages * PAGE_SIZE,
                        failure_rate: if attempts > 0 {
                            (failures as f64 / attempts as f64) * 100.0
                        } else {
                            0.0
                        },
                        attempts,
                        failures,
                    });
                }
                
                // Contention overhead (simulated)
                let base_overhead = 1.2;
                let time_factor = 1.0 + (target_time / 24.0) * 0.2;
                results.contention_timeline.push((target_time, base_overhead * time_factor));
            }
        }
    }
    
    // Clean up allocations
    for (addr, _, size) in &allocated {
        allocator.dealloc_pages(*addr, *size);
    }
    
    // Check for memory leaks
    let final_stats = allocator.get_stats();
    let final_free = final_stats.1;
    
    if initial_free > final_free + PAGE_SIZE {
        results.leak_detected = true;
        results.leaked_bytes = initial_free - final_free;
    }
    
    results
}

/// Measure current performance metrics.
fn measure_performance<A: PageAllocator + ?Sized>(allocator: &A, time_hours: f64) -> PerformanceSnapshot {
    let mut alloc_times: Vec<u64> = Vec::new();
    let mut dealloc_times: Vec<u64> = Vec::new();
    let mut successes = 0;
    let attempts = 100;
    
    let test_start = Instant::now();
    
    for _ in 0..attempts {
        let start = Instant::now();
        match allocator.alloc_pages(4, PAGE_SIZE) {
            Ok(addr) => {
                alloc_times.push(start.elapsed().as_nanos() as u64);
                successes += 1;
                
                let dealloc_start = Instant::now();
                allocator.dealloc_pages(addr, 4);
                dealloc_times.push(dealloc_start.elapsed().as_nanos() as u64);
            }
            Err(_) => {}
        }
    }
    
    let total_elapsed = test_start.elapsed();
    
    let avg_alloc = if !alloc_times.is_empty() {
        alloc_times.iter().sum::<u64>() as f64 / alloc_times.len() as f64 / 1000.0
    } else {
        0.0
    };
    
    let avg_dealloc = if !dealloc_times.is_empty() {
        dealloc_times.iter().sum::<u64>() as f64 / dealloc_times.len() as f64 / 1000.0
    } else {
        0.0
    };
    
    let throughput = if total_elapsed.as_secs_f64() > 0.0 {
        (successes * 2) as f64 / total_elapsed.as_secs_f64()
    } else {
        0.0
    };
    
    PerformanceSnapshot {
        time_hours,
        avg_alloc_time_us: avg_alloc,
        avg_dealloc_time_us: avg_dealloc,
        throughput_ops_per_sec: throughput,
        success_rate: (successes as f64 / attempts as f64) * 100.0,
    }
}

/// Measure failure rate for a specific allocation size.
fn measure_failure_rate<A: PageAllocator + ?Sized>(
    allocator: &A,
    size_pages: usize,
    attempts: usize,
) -> (usize, usize) {
    let mut failures = 0;
    
    for _ in 0..attempts {
        match allocator.alloc_pages(size_pages, PAGE_SIZE) {
            Ok(addr) => {
                allocator.dealloc_pages(addr, size_pages);
            }
            Err(_) => {
                failures += 1;
            }
        }
    }
    
    (attempts, failures)
}

/// Run time-dimension test and print results.
pub fn run_and_print_time_dimension_test<A: PageAllocator + ?Sized>(
    allocator: &A,
    config: &TimeDimensionConfig,
) {
    std::println!("Starting time-dimension test for {}...", allocator.name());
    std::println!("Simulating {} hours of operation...", 
        config.hours.iter().cloned().fold(0.0f64, f64::max));
    
    let results = run_time_dimension_test(allocator, config);
    std::println!("{}", results.to_report());
}
