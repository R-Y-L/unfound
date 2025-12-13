//! Hybrid allocator combining free-lists (for large blocks) and bitmap (for small blocks).
//!
//! Strategy:
//! - Blocks >= `THRESHOLD_PAGES` (e.g., 64 pages) are managed by free-list (buddy-like merging).
//! - Blocks < `THRESHOLD_PAGES` are managed by bitmap for fine-grained allocation.
//! - This reduces fragmentation for small allocations while keeping large allocations efficient.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use allocator::AllocError;
use kspin::SpinNoIrq;
use memory_addr::is_aligned;
use super::PageAllocator;

const PAGE_SIZE: usize = 4096;
const THRESHOLD_PAGES: usize = 64; // Blocks >= 64 pages use free-list; smaller use bitmap

/// Helper struct for free-list entry (large block).
#[derive(Clone, Debug)]
struct FreeBlockInfo {
    size: usize, // in pages
}

pub struct HybridAllocator {
    base: usize,
    total_pages: usize,
    
    /// Bitmap for small allocations: 1 bit per page, 1 = free, 0 = allocated.
    bitmap: SpinNoIrq<Vec<u8>>,
    
    /// Free-list for large blocks: page_index -> block_size (in pages).
    free_list: SpinNoIrq<BTreeMap<usize, FreeBlockInfo>>,
    
    /// Track allocations: start_index -> (size_in_pages, is_large).
    alloc_map: SpinNoIrq<BTreeMap<usize, (usize, bool)>>,
    
    used_pages: SpinNoIrq<usize>,
}

impl HybridAllocator {
    pub fn new() -> Self {
        Self {
            base: 0,
            total_pages: 0,
            bitmap: SpinNoIrq::new(Vec::new()),
            free_list: SpinNoIrq::new(BTreeMap::new()),
            alloc_map: SpinNoIrq::new(BTreeMap::new()),
            used_pages: SpinNoIrq::new(0),
        }
    }

    /// Mark pages in bitmap as free (bit = 1).
    fn mark_free(&self, start_idx: usize, count: usize) {
        let mut bitmap = self.bitmap.lock();
        for i in start_idx..start_idx + count {
            if i < self.total_pages {
                let byte_idx = i / 8;
                let bit_idx = i % 8;
                bitmap[byte_idx] |= 1u8 << bit_idx;
            }
        }
    }

    /// Mark pages in bitmap as allocated (bit = 0).
    fn mark_allocated(&self, start_idx: usize, count: usize) {
        let mut bitmap = self.bitmap.lock();
        for i in start_idx..start_idx + count {
            if i < self.total_pages {
                let byte_idx = i / 8;
                let bit_idx = i % 8;
                bitmap[byte_idx] &= !(1u8 << bit_idx);
            }
        }
    }

    /// Find first free bit in bitmap.
    fn find_free_in_bitmap(&self, needed: usize) -> Option<usize> {
        let bitmap = self.bitmap.lock();
        for start in 0..self.total_pages - needed + 1 {
            let mut all_free = true;
            for i in start..start + needed {
                let byte_idx = i / 8;
                let bit_idx = i % 8;
                if (bitmap[byte_idx] & (1u8 << bit_idx)) == 0 {
                    all_free = false;
                    break;
                }
            }
            if all_free {
                return Some(start);
            }
        }
        None
    }

    /// Find first free block in free-list that fits the requested size.
    fn find_free_block(&self, needed_pages: usize) -> Option<(usize, usize)> {
        let free_list = self.free_list.lock();
        for (&idx, info) in free_list.iter() {
            if info.size >= needed_pages {
                return Some((idx, info.size));
            }
        }
        None
    }

    /// Split a large block if it's larger than needed.
    fn split_block(&self, start_idx: usize, original_size: usize, needed_size: usize) {
        if original_size > needed_size {
            let remaining_start = start_idx + needed_size;
            let remaining_size = original_size - needed_size;
            self.free_list.lock().insert(remaining_start, FreeBlockInfo {
                size: remaining_size,
            });
        }
    }

    /// Try to merge adjacent free blocks.
    fn try_merge(&self, start_idx: usize, size: usize) {
        let mut free_list = self.free_list.lock();
        let end_idx = start_idx + size;

        // Try merging with block before
        if let Some((&prev_idx, prev_info)) = free_list.range(..start_idx).next_back() {
            if prev_idx + prev_info.size == start_idx {
                let prev_size = prev_info.size;
                free_list.remove(&prev_idx);
                free_list.insert(prev_idx, FreeBlockInfo {
                    size: prev_size + size,
                });
                return;
            }
        }

        // Try merging with block after
        if let Some((&next_idx, next_info)) = free_list.range(end_idx..).next() {
            if next_idx == end_idx {
                let next_size = next_info.size;
                free_list.remove(&next_idx);
                free_list.insert(start_idx, FreeBlockInfo {
                    size: size + next_size,
                });
            }
        }
    }
}

impl PageAllocator for HybridAllocator {
    fn name(&self) -> &'static str {
        "hybrid"
    }

    fn init(&self, start_vaddr: usize, size: usize) -> Result<(), AllocError> {
        let end = (start_vaddr + size) & !(PAGE_SIZE - 1);
        let start = (start_vaddr + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        if end <= start {
            return Err(AllocError::InvalidParam);
        }
        let total_pages = (end - start) / PAGE_SIZE;
        if total_pages == 0 {
            return Err(AllocError::InvalidParam);
        }

        // Initialize bitmap: all pages are free (bit = 1)
        let bitmap_size = (total_pages + 7) / 8;
        let bitmap = {
            let mut vec = Vec::new();
            vec.resize(bitmap_size, 0xFFu8);
            vec
        };
        let mut bitmap = bitmap;
        if total_pages % 8 != 0 {
            let last_byte_idx = bitmap_size - 1;
            let unused_bits = 8 - (total_pages % 8);
            bitmap[last_byte_idx] &= 0xFFu8 >> unused_bits;
        }

        // All memory starts as one large free block
        let mut free_list = BTreeMap::new();
        free_list.insert(0, FreeBlockInfo { size: total_pages });

        let mut bitmap_guard = self.bitmap.lock();
        *bitmap_guard = bitmap;
        drop(bitmap_guard);

        let mut fl = self.free_list.lock();
        *fl = free_list;
        drop(fl);

        self.alloc_map.lock().clear();
        *self.used_pages.lock() = 0;

        unsafe {
            let s = self as *const Self as *mut Self;
            (*s).base = start;
            (*s).total_pages = total_pages;
        }

        Ok(())
    }

    fn alloc_pages(&self, num_pages: usize, align_pow2: usize) -> Result<usize, AllocError> {
        if num_pages == 0 {
            return Err(AllocError::InvalidParam);
        }
        if align_pow2 < PAGE_SIZE || !align_pow2.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }

        // Determine if we use free-list (large) or bitmap (small)
        if num_pages >= THRESHOLD_PAGES {
            // Large allocation: use free-list
            if let Some((block_idx, block_size)) = self.find_free_block(num_pages) {
                let mut free_list = self.free_list.lock();
                free_list.remove(&block_idx);
                drop(free_list);

                // Split if needed
                self.split_block(block_idx, block_size, num_pages);

                // Record allocation
                self.alloc_map.lock().insert(block_idx, (num_pages, true));
                *self.used_pages.lock() += num_pages;

                return Ok(self.base + block_idx * PAGE_SIZE);
            }
        } else {
            // Small allocation: use bitmap
            if let Some(block_idx) = self.find_free_in_bitmap(num_pages) {
                self.mark_allocated(block_idx, num_pages);
                self.alloc_map.lock().insert(block_idx, (num_pages, false));
                *self.used_pages.lock() += num_pages;

                return Ok(self.base + block_idx * PAGE_SIZE);
            }
        }

        Err(AllocError::NoMemory)
    }

    fn alloc_pages_at(
        &self,
        start: usize,
        num_pages: usize,
        align_pow2: usize,
    ) -> Result<usize, AllocError> {
        if num_pages == 0 {
            return Err(AllocError::InvalidParam);
        }
        if align_pow2 < PAGE_SIZE || !align_pow2.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }
        if start < self.base || start >= self.base + self.total_pages * PAGE_SIZE {
            return Err(AllocError::InvalidParam);
        }
        if !is_aligned(start, align_pow2) {
            return Err(AllocError::InvalidParam);
        }

        let idx = (start - self.base) / PAGE_SIZE;

        // Try to allocate at the exact location
        if num_pages >= THRESHOLD_PAGES {
            // Large: check free-list
            let mut free_list = self.free_list.lock();
            if let Some(info) = free_list.get(&idx) {
                if info.size >= num_pages {
                    let size = info.size;
                    free_list.remove(&idx);
                    drop(free_list);

                    self.split_block(idx, size, num_pages);
                    self.alloc_map.lock().insert(idx, (num_pages, true));
                    *self.used_pages.lock() += num_pages;

                    return Ok(start);
                }
            }
        } else {
            // Small: check bitmap
            let bitmap = self.bitmap.lock();
            let mut all_free = true;
            for i in idx..idx + num_pages {
                let byte_idx = i / 8;
                let bit_idx = i % 8;
                if (bitmap[byte_idx] & (1u8 << bit_idx)) == 0 {
                    all_free = false;
                    break;
                }
            }
            drop(bitmap);

            if all_free {
                self.mark_allocated(idx, num_pages);
                self.alloc_map.lock().insert(idx, (num_pages, false));
                *self.used_pages.lock() += num_pages;

                return Ok(start);
            }
        }

        Err(AllocError::NoMemory)
    }

    fn dealloc_pages(&self, pos: usize, _num_pages: usize) {
        if pos < self.base || pos >= self.base + self.total_pages * PAGE_SIZE {
            return;
        }
        if !is_aligned(pos, PAGE_SIZE) {
            return;
        }

        let idx = (pos - self.base) / PAGE_SIZE;

        // Look up the allocation
        let alloc_info = match self.alloc_map.lock().remove(&idx) {
            Some(info) => info,
            None => return,
        };

        let (size, is_large) = alloc_info;

        if is_large {
            // Return to free-list and try to merge
            self.free_list.lock().insert(idx, FreeBlockInfo { size });
            self.try_merge(idx, size);
        } else {
            // Return to bitmap
            self.mark_free(idx, size);
        }

        *self.used_pages.lock() -= size;
    }
}
