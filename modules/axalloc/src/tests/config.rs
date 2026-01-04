//! Test configuration and parameters.
//!
//! Defines configurable parameters for test suites to allow customization
//! of test intensity, duration, and patterns.

extern crate alloc;

use alloc::vec::Vec;
use alloc::vec;

/// Configuration for basic performance tests.
#[derive(Debug, Clone)]
pub struct BasicPerfConfig {
    /// Number of allocation attempts for success rate testing.
    pub num_allocations: usize,
    /// Page sizes to test (in number of pages).
    pub allocation_sizes: Vec<usize>,
    /// Alignment requirement (power of 2).
    pub alignment: usize,
    /// Number of warmup iterations before measurement.
    pub warmup_iterations: usize,
    /// Whether to measure individual allocation times (for percentiles).
    pub measure_individual_times: bool,
    /// Ratio of deallocations to perform during allocation phase (0.0 to 1.0).
    /// Higher values mean more mixed alloc/dealloc pattern for better memory reuse.
    pub dealloc_during_alloc_ratio: f64,
}

impl Default for BasicPerfConfig {
    fn default() -> Self {
        Self {
            num_allocations: 10000,
            allocation_sizes: vec![1, 4, 16, 64, 256],
            alignment: 4096,
            warmup_iterations: 100,
            measure_individual_times: true,
            dealloc_during_alloc_ratio: 0.3, // 30% chance to dealloc during alloc phase
        }
    }
}

impl BasicPerfConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Quick test config for fast validation.
    pub fn quick() -> Self {
        Self {
            num_allocations: 1000,
            allocation_sizes: vec![1, 16, 64],
            alignment: 4096,
            warmup_iterations: 10,
            measure_individual_times: false,
            dealloc_during_alloc_ratio: 0.3,
        }
    }

    /// Stress test config for thorough testing.
    pub fn stress() -> Self {
        Self {
            num_allocations: 100000,
            allocation_sizes: vec![1, 2, 4, 8, 16, 32, 64, 128, 256],
            alignment: 4096,
            warmup_iterations: 1000,
            measure_individual_times: true,
            dealloc_during_alloc_ratio: 0.4, // Higher ratio for stress test
        }
    }
}

/// Configuration for fragmentation tests.
#[derive(Debug, Clone)]
pub struct FragmentationConfig {
    /// Time points to measure fragmentation (simulated hours).
    pub measurement_times_hours: Vec<f64>,
    /// Operations per simulated hour.
    pub ops_per_hour: usize,
    /// Ratio of allocations that get deallocated (0.0 - 1.0).
    pub dealloc_ratio: f64,
    /// Random seed for reproducibility.
    pub random_seed: u64,
    /// Mixed allocation pattern: (size_pages, weight).
    pub size_distribution: Vec<(usize, usize)>,
}

impl Default for FragmentationConfig {
    fn default() -> Self {
        Self {
            measurement_times_hours: vec![0.0, 1.0, 6.0, 24.0],
            ops_per_hour: 10000,
            dealloc_ratio: 0.7,
            random_seed: 12345,
            size_distribution: vec![
                (1, 50),    // 50% single page
                (4, 20),    // 20% 4 pages (16KB)
                (16, 15),   // 15% 16 pages (64KB)
                (64, 10),   // 10% 64 pages (256KB)
                (256, 5),   // 5% 256 pages (1MB)
            ],
        }
    }
}

impl FragmentationConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Quick fragmentation test.
    pub fn quick() -> Self {
        Self {
            measurement_times_hours: vec![0.0, 1.0],
            ops_per_hour: 1000,
            dealloc_ratio: 0.7,
            random_seed: 12345,
            size_distribution: vec![
                (1, 60),
                (16, 30),
                (64, 10),
            ],
        }
    }
}

/// Configuration for stability tests.
#[derive(Debug, Clone)]
pub struct StabilityConfig {
    /// Sizes to test for failure rate (in bytes).
    pub test_sizes_bytes: Vec<usize>,
    /// Time points to measure failure rate (simulated hours).
    pub measurement_times_hours: Vec<f64>,
    /// Operations per simulated hour for failure rate test.
    pub ops_per_hour: usize,
    /// Number of iterations for leak test.
    pub leak_test_iterations: usize,
    /// Simulated duration for leak test (hours).
    pub leak_test_duration_hours: f64,
    /// Number of threads for contention test.
    pub num_threads: usize,
    /// Operations per thread for contention test.
    pub ops_per_thread: usize,
}

impl Default for StabilityConfig {
    fn default() -> Self {
        Self {
            test_sizes_bytes: vec![16 * 1024, 1024 * 1024], // 16KB, 1MB
            measurement_times_hours: vec![1.0, 6.0, 24.0],
            ops_per_hour: 10000,
            leak_test_iterations: 100000,
            leak_test_duration_hours: 24.0,
            num_threads: 4,
            ops_per_thread: 10000,
        }
    }
}

impl StabilityConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Quick stability test.
    pub fn quick() -> Self {
        Self {
            test_sizes_bytes: vec![16 * 1024, 1024 * 1024],
            measurement_times_hours: vec![1.0],
            ops_per_hour: 1000,
            leak_test_iterations: 10000,
            leak_test_duration_hours: 1.0,
            num_threads: 2,
            ops_per_thread: 1000,
        }
    }
}

/// Configuration for runtime switch tests.
#[derive(Debug, Clone)]
pub struct RuntimeSwitchConfig {
    /// Number of switch iterations to measure latency variance.
    pub num_switch_iterations: usize,
    /// Allocations to make before switching.
    pub pre_switch_allocations: usize,
    /// Duration to run after switch before measuring recovery (simulated minutes).
    pub recovery_wait_minutes: f64,
    /// Source allocator name.
    pub from_allocator: &'static str,
    /// Target allocator name.
    pub to_allocator: &'static str,
}

impl Default for RuntimeSwitchConfig {
    fn default() -> Self {
        Self {
            num_switch_iterations: 10,
            pre_switch_allocations: 5000,
            recovery_wait_minutes: 10.0,
            from_allocator: "buddy",
            to_allocator: "hybrid",
        }
    }
}

impl RuntimeSwitchConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Master configuration combining all test configs.
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// Total memory pool size for testing (bytes).
    pub memory_pool_size: usize,
    /// Starting virtual address.
    pub start_vaddr: usize,
    /// Page size.
    pub page_size: usize,
    /// Basic performance config.
    pub basic_perf: BasicPerfConfig,
    /// Fragmentation config.
    pub fragmentation: FragmentationConfig,
    /// Stability config.
    pub stability: StabilityConfig,
    /// Runtime switch config.
    pub runtime_switch: RuntimeSwitchConfig,
    /// Enable verbose output.
    pub verbose: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            memory_pool_size: 1024 * 1024 * 1024, // 1GB
            start_vaddr: 0x1000,
            page_size: 4096,
            basic_perf: BasicPerfConfig::default(),
            fragmentation: FragmentationConfig::default(),
            stability: StabilityConfig::default(),
            runtime_switch: RuntimeSwitchConfig::default(),
            verbose: false,
        }
    }
}

impl TestConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Small memory pool for quick tests.
    pub fn small() -> Self {
        Self {
            memory_pool_size: 64 * 1024 * 1024, // 64MB
            start_vaddr: 0x1000,
            page_size: 4096,
            basic_perf: BasicPerfConfig::quick(),
            fragmentation: FragmentationConfig::quick(),
            stability: StabilityConfig::quick(),
            runtime_switch: RuntimeSwitchConfig::default(),
            verbose: false,
        }
    }

    /// Medium memory pool for standard tests.
    pub fn medium() -> Self {
        Self {
            memory_pool_size: 256 * 1024 * 1024, // 256MB
            ..Self::default()
        }
    }

    /// Large memory pool for stress tests.
    pub fn large() -> Self {
        Self {
            memory_pool_size: 1024 * 1024 * 1024, // 1GB
            basic_perf: BasicPerfConfig::stress(),
            ..Self::default()
        }
    }
}
