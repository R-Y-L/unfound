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
    use allocator::AllocError;
    use core::option::Option;
    use kspin::SpinNoIrq;

    // Global storage for the runtime-selected page allocator. When `None`,
    // the system falls back to the built-in page allocator.
    static GLOBAL_PAGE_ALLOC: SpinNoIrq<Option<Box<dyn PageAllocator>>> =
        SpinNoIrq::new(None);

    /// Try to set the global runtime allocator. Overwrites any previous value.
    pub fn set_runtime_allocator(a: Box<dyn PageAllocator>) {
        let mut slot = GLOBAL_PAGE_ALLOC.lock();
        *slot = Some(a);
    }

    /// Clear the runtime allocator (revert to built-in fallback).
    pub fn clear_runtime_allocator() {
        let mut slot = GLOBAL_PAGE_ALLOC.lock();
        *slot = None;
    }

    /// Allocate pages via the runtime allocator if present.
    pub fn alloc_pages(num_pages: usize, align_pow2: usize) -> Result<usize, AllocError> {
        let slot = GLOBAL_PAGE_ALLOC.lock();
        if let Some(ref a) = *slot {
            a.alloc_pages(num_pages, align_pow2)
        } else {
            Err(AllocError::NoMemory)
        }
    }

    /// Allocate pages at exact location via runtime allocator if present.
    pub fn alloc_pages_at(start: usize, num_pages: usize, align_pow2: usize) -> Result<usize, AllocError> {
        let slot = GLOBAL_PAGE_ALLOC.lock();
        if let Some(ref a) = *slot {
            a.alloc_pages_at(start, num_pages, align_pow2)
        } else {
            Err(AllocError::NoMemory)
        }
    }

    /// Deallocate pages via the runtime allocator if present.
    pub fn dealloc_pages(pos: usize, num_pages: usize) {
        let slot = GLOBAL_PAGE_ALLOC.lock();
        if let Some(ref a) = *slot {
            a.dealloc_pages(pos, num_pages)
        }
    }

    /// Helper to create an allocator by name. Recognized names: "buddy",
    /// "bitmap", "hybrid". Returns an error if the chosen allocator
    /// is not compiled-in (feature not enabled) or name is unknown.
    pub fn make_by_name(name: &str) -> Result<Box<dyn PageAllocator>, &'static str> {
        match name {
            "buddy" => {
                #[cfg(feature = "buddy")]
                {
                    return Ok(Box::new(crate::allocators::BuddyAllocator::new()));
                }
                #[cfg(not(feature = "buddy"))]
                {
                    return Err("buddy feature not enabled");
                }
            }
            "bitmap" => {
                #[cfg(feature = "bitmap")]
                {
                    return Ok(Box::new(crate::allocators::BitmapAllocator::new()));
                }
                #[cfg(not(feature = "bitmap"))]
                {
                    return Err("bitmap feature not enabled");
                }
            }
            "hybrid" => {
                #[cfg(feature = "hybrid")]
                {
                    return Ok(Box::new(crate::allocators::HybridAllocator::new()));
                }
                #[cfg(not(feature = "hybrid"))]
                {
                    return Err("hybrid feature not enabled");
                }
            }
            _ => Err("unknown allocator name"),
        }
    }
}
