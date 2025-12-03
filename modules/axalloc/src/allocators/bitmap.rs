//! Minimal bitmap allocator stub for small-memory embedded targets.
use allocator::AllocError;
use super::PageAllocator;

pub struct BitmapAllocator;

impl BitmapAllocator {
    pub fn new() -> Self {
        Self
    }
}

impl PageAllocator for BitmapAllocator {
    fn name(&self) -> &'static str {
        "bitmap"
    }

    fn init(&self, _start_vaddr: usize, _size: usize) -> Result<(), AllocError> {
        // TODO: implement lightweight bitmap initialization
        Err(AllocError::InvalidParam)
    }

    fn alloc_pages(&self, _num_pages: usize, _align_pow2: usize) -> Result<usize, AllocError> {
        Err(AllocError::NoMemory)
    }

    fn alloc_pages_at(
        &self,
        _start: usize,
        _num_pages: usize,
        _align_pow2: usize,
    ) -> Result<usize, AllocError> {
        Err(AllocError::NoMemory)
    }

    fn dealloc_pages(&self, _pos: usize, _num_pages: usize) {
        // noop
    }
}
