//! Metrics: Data structures for storing and reporting test results.
//!
//! This module defines comprehensive metrics for memory allocator testing,
//! covering performance, fragmentation, stability, and runtime switching.

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

/// Basic performance metrics from a test run.
#[derive(Debug, Clone, Default)]
pub struct BasicPerformanceMetrics {
    /// Total number of allocation attempts.
    pub total_allocations: usize,
    /// Number of successful allocations.
    pub successful_allocations: usize,
    /// Number of failed allocations.
    pub failed_allocations: usize,
    /// Success rate as percentage (0.0 - 100.0).
    pub success_rate: f64,
    /// Average allocation time in microseconds.
    pub avg_alloc_time_us: f64,
    /// Average deallocation time in microseconds.
    pub avg_dealloc_time_us: f64,
    /// Throughput in operations per second (alloc + dealloc combined).
    pub throughput_ops_per_sec: f64,
    /// Minimum allocation time in microseconds.
    pub min_alloc_time_us: f64,
    /// Maximum allocation time in microseconds.
    pub max_alloc_time_us: f64,
    /// P50 allocation time in microseconds.
    pub p50_alloc_time_us: f64,
    /// P99 allocation time in microseconds.
    pub p99_alloc_time_us: f64,
}

impl BasicPerformanceMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate success rate.
    pub fn calculate_success_rate(&mut self) {
        if self.total_allocations > 0 {
            self.success_rate = (self.successful_allocations as f64 / self.total_allocations as f64) * 100.0;
        }
    }

    /// Format as human-readable report.
    pub fn to_report(&self) -> String {
        // Use appropriate time unit based on magnitude
        let (alloc_time, alloc_unit) = format_time_us(self.avg_alloc_time_us);
        let (dealloc_time, dealloc_unit) = format_time_us(self.avg_dealloc_time_us);
        let (p50_time, p50_unit) = format_time_us(self.p50_alloc_time_us);
        let (p99_time, p99_unit) = format_time_us(self.p99_alloc_time_us);
        
        format!(
            "【基础性能】\n\
             - 分配成功率：{:.1}%（尝试{}次，成功{}次）\n\
             - 平均分配时间：{:.2}{}\n\
             - 平均释放时间：{:.2}{}\n\
             - 吞吐量：{:.0} 次/秒\n\
             - P50延迟：{:.2}{}，P99延迟：{:.2}{}",
            self.success_rate,
            self.total_allocations,
            self.successful_allocations,
            alloc_time, alloc_unit,
            dealloc_time, dealloc_unit,
            self.throughput_ops_per_sec,
            p50_time, p50_unit,
            p99_time, p99_unit
        )
    }
}

/// Format time in microseconds to appropriate unit (ns, μs, ms).
fn format_time_us(time_us: f64) -> (f64, &'static str) {
    if time_us < 0.001 {
        // Less than 1 nanosecond - show as ns
        (time_us * 1000.0, "ns")
    } else if time_us < 1.0 {
        // Less than 1 microsecond - show as ns
        (time_us * 1000.0, "ns")
    } else if time_us < 1000.0 {
        // Less than 1 millisecond - show as μs
        (time_us, "μs")
    } else {
        // 1 millisecond or more - show as ms
        (time_us / 1000.0, "ms")
    }
}

/// Fragmentation snapshot at a specific time.
#[derive(Debug, Clone, Default)]
pub struct FragmentationSnapshot {
    /// Time offset in hours from start (0, 1, 6, 24).
    pub time_hours: f64,
    /// Total free memory in bytes.
    pub total_free_bytes: usize,
    /// Largest contiguous free block in bytes.
    pub max_contiguous_free_bytes: usize,
    /// External fragmentation rate as percentage.
    pub external_fragmentation_rate: f64,
    /// Total allocated memory in bytes.
    pub total_allocated_bytes: usize,
    /// Wasted memory due to internal fragmentation.
    pub internal_waste_bytes: usize,
    /// Internal fragmentation rate as percentage.
    pub internal_fragmentation_rate: f64,
}

impl FragmentationSnapshot {
    /// Calculate fragmentation rates from raw values.
    pub fn calculate_rates(&mut self) {
        if self.total_free_bytes > 0 {
            self.external_fragmentation_rate = 
                (1.0 - (self.max_contiguous_free_bytes as f64 / self.total_free_bytes as f64)) * 100.0;
        }
        if self.total_allocated_bytes > 0 {
            self.internal_fragmentation_rate = 
                (self.internal_waste_bytes as f64 / self.total_allocated_bytes as f64) * 100.0;
        }
    }
}

/// Fragmentation metrics over time.
#[derive(Debug, Clone, Default)]
pub struct FragmentationMetrics {
    /// Snapshots at different time points.
    pub snapshots: Vec<FragmentationSnapshot>,
    /// Metadata overhead in bytes.
    pub metadata_overhead_bytes: usize,
    /// Total memory in bytes (for calculating overhead percentage).
    pub total_memory_bytes: usize,
}

impl FragmentationMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn metadata_overhead_percentage(&self) -> f64 {
        if self.total_memory_bytes > 0 {
            (self.metadata_overhead_bytes as f64 / self.total_memory_bytes as f64) * 100.0
        } else {
            0.0
        }
    }

    pub fn to_report(&self) -> String {
        let mut report = String::from("【碎片与开销（时间维度）】\n- 外部碎片率：\n");
        for snap in &self.snapshots {
            report.push_str(&format!(
                "  - 运行{:.0}h：{:.1}%（总空闲{}，最大连续{}）\n",
                snap.time_hours,
                snap.external_fragmentation_rate,
                format_bytes(snap.total_free_bytes),
                format_bytes(snap.max_contiguous_free_bytes)
            ));
        }
        report.push_str("- 内部碎片率：\n");
        for snap in &self.snapshots {
            report.push_str(&format!(
                "  - 运行{:.0}h：{:.1}%（已分配{}，浪费{}）\n",
                snap.time_hours,
                snap.internal_fragmentation_rate,
                format_bytes(snap.total_allocated_bytes),
                format_bytes(snap.internal_waste_bytes)
            ));
        }
        report.push_str(&format!(
            "- 元数据占用：{}（总内存{}，占比{:.4}%）",
            format_bytes(self.metadata_overhead_bytes),
            format_bytes(self.total_memory_bytes),
            self.metadata_overhead_percentage()
        ));
        report
    }
}

/// Format bytes to appropriate unit (B, KB, MB, GB).
fn format_bytes(bytes: usize) -> String {
    if bytes == 0 {
        "0B".to_string()
    } else if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2}GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Failure rate snapshot by allocation size.
#[derive(Debug, Clone, Default)]
pub struct FailureRateSnapshot {
    /// Time offset in hours.
    pub time_hours: f64,
    /// Allocation size in bytes.
    pub size_bytes: usize,
    /// Failure rate as percentage.
    pub failure_rate: f64,
    /// Total attempts.
    pub attempts: usize,
    /// Failed attempts.
    pub failures: usize,
}

/// Stability metrics over time.
#[derive(Debug, Clone, Default)]
pub struct StabilityMetrics {
    /// Failure rates by size and time.
    pub failure_rates: Vec<FailureRateSnapshot>,
    /// Memory leak rate in bytes per second.
    pub memory_leak_rate_bps: f64,
    /// Multi-thread contention overhead ratios over time.
    pub contention_ratios: Vec<(f64, f64)>, // (time_hours, ratio)
    /// Performance degradation over time.
    pub alloc_time_degradation: Vec<(f64, f64)>, // (time_hours, avg_time_us)
    /// Throughput over time.
    pub throughput_over_time: Vec<(f64, f64)>, // (time_hours, ops_per_sec)
    /// Loop iterations for leak test.
    pub leak_test_iterations: usize,
    /// Duration in hours for leak test.
    pub leak_test_duration_hours: f64,
}

impl StabilityMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_report(&self) -> String {
        let mut report = String::from("【稳定性（时间维度）】\n- 分配失败率（按大小+时间）：\n");
        
        // Group by size
        let mut sizes: Vec<usize> = self.failure_rates.iter().map(|f| f.size_bytes).collect();
        sizes.sort();
        sizes.dedup();
        
        for size in sizes {
            let entries: Vec<_> = self.failure_rates.iter()
                .filter(|f| f.size_bytes == size)
                .collect();
            if !entries.is_empty() {
                let size_str = if size >= 1024 * 1024 {
                    format!("{}MB", size / (1024 * 1024))
                } else {
                    format!("{}KB", size / 1024)
                };
                report.push_str(&format!("  - {}：", size_str));
                for (i, e) in entries.iter().enumerate() {
                    if i > 0 { report.push_str(" → "); }
                    report.push_str(&format!("运行{:.0}h({:.0}%)", e.time_hours, e.failure_rate));
                }
                report.push('\n');
            }
        }
        
        report.push_str(&format!(
            "- 内存泄漏率：{:.1} B/s（循环{}次/持续{:.0}h运行）\n",
            self.memory_leak_rate_bps,
            self.leak_test_iterations,
            self.leak_test_duration_hours
        ));
        
        report.push_str("- 多线程竞争开销：\n");
        for (time, ratio) in &self.contention_ratios {
            report.push_str(&format!("  - 运行{:.0}h：{:.1}倍（4线程 vs 单线程）\n", time, ratio));
        }
        
        report.push_str("- 性能衰减率：\n");
        if !self.alloc_time_degradation.is_empty() {
            let first = self.alloc_time_degradation.first().unwrap();
            let last = self.alloc_time_degradation.last().unwrap();
            let degradation = ((last.1 - first.1) / first.1) * 100.0;
            report.push_str(&format!(
                "  - 平均分配时间衰减：运行{:.0}h({:.2}ms) → 运行{:.0}h({:.2}ms)（衰减{:.0}%）\n",
                first.0, first.1 / 1000.0,
                last.0, last.1 / 1000.0,
                degradation
            ));
        }
        if !self.throughput_over_time.is_empty() {
            let first = self.throughput_over_time.first().unwrap();
            let last = self.throughput_over_time.last().unwrap();
            let degradation = ((first.1 - last.1) / first.1) * 100.0;
            report.push_str(&format!(
                "  - 吞吐量衰减：运行{:.0}h({:.0}次/秒) → 运行{:.0}h({:.0}次/秒)（衰减{:.0}%）",
                first.0, first.1,
                last.0, last.1,
                degradation
            ));
        }
        
        report
    }
}

/// Runtime switching metrics.
#[derive(Debug, Clone, Default)]
pub struct RuntimeSwitchMetrics {
    /// Switch latency in milliseconds.
    pub switch_latency_ms: f64,
    /// Min switch latency across multiple switches.
    pub min_switch_latency_ms: f64,
    /// Max switch latency across multiple switches.
    pub max_switch_latency_ms: f64,
    /// Number of switches tested.
    pub num_switches: usize,
    /// Compatibility rate (100% = all memory accessible after switch).
    pub compatibility_rate: f64,
    /// Performance before switch.
    pub pre_switch_fragmentation: f64,
    pub pre_switch_throughput: f64,
    /// Performance after switch (recovery).
    pub post_switch_fragmentation: f64,
    pub post_switch_throughput: f64,
    /// Source allocator name.
    pub from_allocator: String,
    /// Target allocator name.
    pub to_allocator: String,
}

impl RuntimeSwitchMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_report(&self) -> String {
        format!(
            "【运行时切换（时间维度）】\n\
             - 切换耗时：{:.0}ms（{} → {}）｜多次切换（{}次）耗时稳定在{:.0}~{:.0}ms\n\
             - 切换后兼容性：{:.0}%（原有内存正常访问/释放）\n\
             - 切换后性能恢复：\n\
             - 切换前：碎片率{:.0}%、吞吐量{:.0}次/秒\n\
             - 切换后：碎片率降至{:.0}%、吞吐量恢复至{:.0}次/秒",
            self.switch_latency_ms,
            self.from_allocator,
            self.to_allocator,
            self.num_switches,
            self.min_switch_latency_ms,
            self.max_switch_latency_ms,
            self.compatibility_rate,
            self.pre_switch_fragmentation,
            self.pre_switch_throughput,
            self.post_switch_fragmentation,
            self.post_switch_throughput
        )
    }
}

/// Complete test report combining all metrics.
#[derive(Debug, Clone, Default)]
pub struct CompleteTestReport {
    /// Allocator name being tested.
    pub allocator_name: String,
    /// Basic performance metrics.
    pub basic_performance: BasicPerformanceMetrics,
    /// Fragmentation metrics.
    pub fragmentation: FragmentationMetrics,
    /// Stability metrics.
    pub stability: StabilityMetrics,
    /// Runtime switch metrics (optional).
    pub runtime_switch: Option<RuntimeSwitchMetrics>,
}

impl CompleteTestReport {
    pub fn new(allocator_name: &str) -> Self {
        Self {
            allocator_name: String::from(allocator_name),
            ..Default::default()
        }
    }

    pub fn to_full_report(&self) -> String {
        let mut report = format!("========== {} 分配器测试报告 ==========\n\n", self.allocator_name);
        report.push_str(&self.basic_performance.to_report());
        report.push_str("\n\n");
        report.push_str(&self.fragmentation.to_report());
        report.push_str("\n\n");
        report.push_str(&self.stability.to_report());
        if let Some(ref switch) = self.runtime_switch {
            report.push_str("\n\n");
            report.push_str(&switch.to_report());
        }
        report.push_str("\n\n==========================================\n");
        report
    }
}
