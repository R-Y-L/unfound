//! Bitmap allocator wrapper for embedded/host use.
//!
//! This wraps the `allocator::BitmapPageAllocator` from the shared
//! `allocator` crate to provide a `PageAllocator`-compatible implementation
//! usable for runtime selection. It's lightweight and mirrors the behavior
//! of the existing page allocator used by `GlobalAllocator`.

use allocator::{AllocError, BitmapPageAllocator};
use kspin::SpinNoIrq;
use super::PageAllocator;

const PAGE_SIZE: usize = 4096;

pub struct BitmapAllocator {
    inner: SpinNoIrq<BitmapPageAllocator<PAGE_SIZE>>,
}

impl BitmapAllocator {
    pub fn new() -> Self {
        Self {
            inner: SpinNoIrq::new(BitmapPageAllocator::new()),
        }
    }
}

impl PageAllocator for BitmapAllocator {
    fn name(&self) -> &'static str {
        "bitmap"
    }

    fn init(&self, start_vaddr: usize, size: usize) -> Result<(), AllocError> {
        self.inner.lock().init(start_vaddr, size);
        Ok(())
    }

    fn alloc_pages(&self, num_pages: usize, align_pow2: usize) -> Result<usize, AllocError> {
        self.inner.lock().alloc_pages(num_pages, align_pow2)
    }

    fn alloc_pages_at(
        &self,
        start: usize,
        num_pages: usize,
        align_pow2: usize,
    ) -> Result<usize, AllocError> {
        self.inner.lock().alloc_pages_at(start, num_pages, align_pow2)
    }

    fn dealloc_pages(&self, pos: usize, num_pages: usize) {
        self.inner.lock().dealloc_pages(pos, num_pages)
    }
}
