//! Hybrid allocator stub combining free-lists and interval indexing.
use allocator::AllocError;
use super::PageAllocator;

pub struct HybridAllocator;

impl HybridAllocator {
    pub fn new() -> Self {
        Self
    }
}

impl PageAllocator for HybridAllocator {
    fn name(&self) -> &'static str {
        "hybrid"
    }

    fn init(&self, _start_vaddr: usize, _size: usize) -> Result<(), AllocError> {
        // TODO: implement hybrid init (free lists + interval tree)
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
