//! Stub wrapper for the buddy allocator implementation.
use allocator::AllocError;
use super::PageAllocator;

pub struct BuddyAllocator;

impl BuddyAllocator {
    pub fn new() -> Self {
        Self
    }
}

impl PageAllocator for BuddyAllocator {
    fn name(&self) -> &'static str {
        "buddy"
    }

    fn init(&self, _start_vaddr: usize, _size: usize) -> Result<(), AllocError> {
        // TODO: move/refactor existing buddy code here
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
