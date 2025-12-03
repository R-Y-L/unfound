//! Allocators module skeleton.
//!
//! This module defines a small `Allocator` trait and conditionally exposes
//! different allocator implementations (buddy/bitmap/hybrid). Implementations
//! are currently stubs — full implementations will be added in follow-up steps.

use allocator::AllocError;

/// Minimal allocator trait for page-level operations used by the runtime
/// switching infrastructure.
pub trait PageAllocator: Send + Sync {
    /// Return a short name of the allocator.
    fn name(&self) -> &'static str;

    /// Initialize allocator with a region starting at `start_vaddr` of `size` bytes.
    fn init(&self, start_vaddr: usize, size: usize) -> Result<(), AllocError>;

    /// Allocate contiguous pages: returns start virtual address on success.
    fn alloc_pages(&self, num_pages: usize, align_pow2: usize) -> Result<usize, AllocError>;

    /// Allocate contiguous pages at an exact start address.
    fn alloc_pages_at(
        &self,
        start: usize,
        num_pages: usize,
        align_pow2: usize,
    ) -> Result<usize, AllocError>;

    /// Deallocate contiguous pages starting from `pos`.
    fn dealloc_pages(&self, pos: usize, num_pages: usize);
}

#[cfg(feature = "buddy")]
mod buddy;
#[cfg(feature = "buddy")]
pub use buddy::BuddyAllocator;

#[cfg(feature = "bitmap")]
mod bitmap;
#[cfg(feature = "bitmap")]
pub use bitmap::BitmapAllocator;

#[cfg(feature = "hybrid")]
mod hybrid;
#[cfg(feature = "hybrid")]
pub use hybrid::HybridAllocator;

// When runtime switching is enabled, compile helpers to build dynamic dispatch
// pointers. The full runtime switcher will be implemented in following steps.
#[cfg(feature = "runtime-switch")]
pub mod runtime {
    use super::PageAllocator;
    use core::sync::atomic::{AtomicBool, Ordering};

    // Placeholder for runtime switching machinery.
    static RUNTIME_SWITCH_ENABLED: AtomicBool = AtomicBool::new(true);

    pub fn runtime_enabled() -> bool {
        RUNTIME_SWITCH_ENABLED.load(Ordering::Relaxed)
    }
}
