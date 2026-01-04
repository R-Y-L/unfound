//! Buddy page allocator implementation.
//!
//! Implements a simple buddy allocator for page-granularity allocation.
//! - Supports allocation sizes rounded up to the next power-of-two number of pages.
//! - Tracks allocations in a map so deallocation frees the full allocated block.
//! - Supports `alloc_pages`, `alloc_pages_at` (exact start), and `dealloc_pages`.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use allocator::AllocError;
use core::cmp;
use kspin::SpinNoIrq;
use memory_addr::is_aligned;
use super::PageAllocator;

const PAGE_SIZE: usize = 4096;

pub struct BuddyAllocator {
    base: usize,
    total_pages: usize,
    max_order: usize,
    /// free_lists[order] contains start indices (in pages) of free blocks of size 2^order
    free_lists: SpinNoIrq<Vec<Vec<usize>>>,
    /// allocation map: start_index -> (order, original_num_pages)
    alloc_map: SpinNoIrq<BTreeMap<usize, (usize, usize)>>,
    used_pages: SpinNoIrq<usize>,
}

fn ceil_log2(n: usize) -> usize {
    if n <= 1 { return 0; }
    let mut v = 1usize;
    let mut r = 0usize;
    while v < n {
        v <<= 1;
        r += 1;
    }
    r
}

impl BuddyAllocator {
    pub fn new() -> Self {
        Self {
            base: 0,
            total_pages: 0,
            max_order: 0,
            free_lists: SpinNoIrq::new(Vec::new()),
            alloc_map: SpinNoIrq::new(BTreeMap::new()),
            used_pages: SpinNoIrq::new(0),
        }
    }

    fn push_free(&self, order: usize, idx: usize) {
        let mut lists = self.free_lists.lock();
        if order >= lists.len() {
            lists.resize(order + 1, Vec::new());
        }
        lists[order].push(idx);
    }

    fn pop_free(&self, order: usize) -> Option<usize> {
        let mut lists = self.free_lists.lock();
        if order >= lists.len() { return None; }
        lists[order].pop()
    }

    fn remove_free_exact(&self, order: usize, idx: usize) -> bool {
        let mut lists = self.free_lists.lock();
        if order >= lists.len() { return false; }
        if let Some(pos) = lists[order].iter().position(|&x| x == idx) {
            lists[order].swap_remove(pos);
            true
        } else { false }
    }
}

impl PageAllocator for BuddyAllocator {
    fn name(&self) -> &'static str { "buddy" }

    fn init(&self, start_vaddr: usize, size: usize) -> Result<(), AllocError> {
        let end = (start_vaddr + size) & !(PAGE_SIZE - 1);
        let start = (start_vaddr + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        if end <= start { return Err(AllocError::InvalidParam); }
        let total_pages = (end - start) / PAGE_SIZE;
        if total_pages == 0 { return Err(AllocError::InvalidParam); }

        let mut mo = 0usize;
        while (1usize << (mo + 1)) <= total_pages { mo += 1; }

        {
            let mut lists = self.free_lists.lock();
            lists.clear();
            lists.resize(mo + 1, Vec::new());
        }
        self.alloc_map.lock().clear();
        *self.used_pages.lock() = 0;

        let mut remaining = total_pages;
        let mut offset = 0usize;
        while remaining > 0 {
            let order = (usize::BITS as usize - 1) - (remaining.leading_zeros() as usize);
            let block_size = 1usize << order;
            self.push_free(order, offset);
            offset += block_size;
            remaining -= block_size;
        }

        unsafe {
            let s = self as *const Self as *mut Self;
            (*s).base = start;
            (*s).total_pages = total_pages;
            (*s).max_order = mo;
        }

        Ok(())
    }

    fn alloc_pages(&self, num_pages: usize, align_pow2: usize) -> Result<usize, AllocError> {
        if num_pages == 0 { return Err(AllocError::InvalidParam); }
        if align_pow2 < PAGE_SIZE || !align_pow2.is_power_of_two() { return Err(AllocError::InvalidParam); }

        let needed = num_pages.next_power_of_two();
        let order = ceil_log2(needed);
        let mut o = order;
        while o <= self.max_order {
            if let Some(idx) = self.pop_free(o) {
                let cur_idx = idx;
                let mut cur_order = o;
                while cur_order > order {
                    cur_order -= 1;
                    let buddy_idx = cur_idx + (1usize << cur_order);
                    self.push_free(cur_order, buddy_idx);
                }
                self.alloc_map.lock().insert(cur_idx, (order, num_pages));
                *self.used_pages.lock() += 1usize << order;
                return Ok(self.base + cur_idx * PAGE_SIZE);
            }
            o += 1;
        }
        Err(AllocError::NoMemory)
    }

    fn alloc_pages_at(&self, start: usize, num_pages: usize, align_pow2: usize) -> Result<usize, AllocError> {
        if num_pages == 0 { return Err(AllocError::InvalidParam); }
        if align_pow2 < PAGE_SIZE || !align_pow2.is_power_of_two() { return Err(AllocError::InvalidParam); }
        if start < self.base || start >= self.base + self.total_pages * PAGE_SIZE { return Err(AllocError::InvalidParam); }
        if !is_aligned(start, align_pow2) { return Err(AllocError::InvalidParam); }
        let idx = (start - self.base) / PAGE_SIZE;
        let needed = num_pages.next_power_of_two();
        let order = ceil_log2(needed);
        if self.remove_free_exact(order, idx) {
            *self.used_pages.lock() += 1usize << order;
            self.alloc_map.lock().insert(idx, (order, num_pages));
            return Ok(start);
        }
        Err(AllocError::NoMemory)
    }

    fn dealloc_pages(&self, pos: usize, _num_pages: usize) {
        if pos < self.base {
            return;
        }
        let idx = (pos - self.base) / PAGE_SIZE;
        if let Some((order, _)) = self.alloc_map.lock().remove(&idx) {
            let mut current_idx = idx;
            let mut current_order = order;
            *self.used_pages.lock() -= 1usize << order;

            while current_order < self.max_order {
                let buddy_idx = current_idx ^ (1usize << current_order);
                if self.remove_free_exact(current_order, buddy_idx) {
                    current_idx = cmp::min(current_idx, buddy_idx);
                    current_order += 1;
                } else {
                    break;
                }
            }
            self.push_free(current_order, current_idx);
        }
    }

    fn get_stats(&self) -> (f64, usize) {
        let free_lists = self.free_lists.lock();
        let alloc_map = self.alloc_map.lock();
        let used_pages = *self.used_pages.lock();

        let total_free_pages = self.total_pages - used_pages;

        let max_free_contiguous_pages = free_lists
            .iter()
            .enumerate()
            .rev()
            .find(|(_, free_list)| !free_list.is_empty())
            .map_or(0, |(order, _)| 1 << order);

        // Calculate fragmentation rate
        let total_free = total_free_pages * PAGE_SIZE;
        let max_contiguous = max_free_contiguous_pages * PAGE_SIZE;
        let fragmentation = if total_free > 0 {
            1.0 - (max_contiguous as f64 / total_free as f64)
        } else {
            0.0
        };

        (fragmentation, total_free)
    }
}

