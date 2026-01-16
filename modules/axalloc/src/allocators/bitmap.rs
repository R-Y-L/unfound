//! Bitmap allocator wrapper for embedded/host use.
//!
//! This wraps the `allocator::BitmapPageAllocator` from the shared
//! `allocator` crate to provide a `PageAllocator`-compatible implementation
//! usable for runtime selection. It's lightweight and mirrors the behavior
//! of the existing page allocator used by `GlobalAllocator`.

extern crate alloc;

use alloc::boxed::Box;
use allocator::{AllocError, BitmapPageAllocator, BaseAllocator, PageAllocator as AllocatorPageAllocator};
use kspin::SpinNoIrq;
use super::PageAllocator;

const PAGE_SIZE: usize = 4096;

pub struct BitmapAllocator {
    // 使用 Box 在堆上分配，避免栈溢出
    inner: SpinNoIrq<Box<BitmapPageAllocator<PAGE_SIZE>>>,
}

impl BitmapAllocator {
    pub fn new() -> Self {
        Self {
            inner: SpinNoIrq::new(Box::new(BitmapPageAllocator::new())),
        }
    }
}

impl PageAllocator for BitmapAllocator {
    fn name(&self) -> &'static str {
        "bitmap"
    }

    fn init(&self, start_vaddr: usize, size: usize) -> Result<(), AllocError> {
        // Debug: 打印初始化参数
        #[cfg(feature = "log")]
        log::debug!("BitmapAllocator::init: start=0x{:x}, size={}", start_vaddr, size);
        
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

    fn get_stats(&self) -> (f64, usize) {
        let inner = self.inner.lock();
        let total_pages = inner.total_pages();
        let used_pages = inner.used_pages();
        let free_pages = total_pages - used_pages;

        // Calculate fragmentation - bitmap doesn't easily track contiguous blocks
        // Estimate fragmentation as 0 for simple bitmap
        let fragmentation = 0.0;
        let total_free_bytes = free_pages * PAGE_SIZE;

        (fragmentation, total_free_bytes)
    }
}
