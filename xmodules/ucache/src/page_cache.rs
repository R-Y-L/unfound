/// 页缓存核心实现

use alloc::vec::Vec;
use lru::LruCache;
use spin::RwLock;
use axerrno::{AxResult, AxError};

/// 缓存页大小（4KB）
pub const PAGE_SIZE: usize = 4096;

/// 缓存页
#[derive(Clone)]
pub struct CachePage {
    pub file_id: usize,
    pub offset: usize,
    pub data: [u8; PAGE_SIZE],
    pub dirty: bool,
}

impl CachePage {
    pub fn new(file_id: usize, offset: usize) -> Self {
        Self {
            file_id,
            offset,
            data: [0u8; PAGE_SIZE],
            dirty: false,
        }
    }
}

/// 缓存键
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct CacheKey {
    file_id: usize,
    page_index: usize,
}

/// 页缓存主结构
pub struct PageCache {
    cache: RwLock<LruCache<CacheKey, CachePage>>,
    hits: core::sync::atomic::AtomicUsize,
    misses: core::sync::atomic::AtomicUsize,
}

impl PageCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: RwLock::new(LruCache::new(core::num::NonZeroUsize::new(capacity).unwrap())),
            hits: core::sync::atomic::AtomicUsize::new(0),
            misses: core::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// 读取页（命中缓存返回，否则加载并缓存）
    pub fn get_page(&self, file_id: usize, offset: usize) -> AxResult<CachePage> {
        let page_index = offset / PAGE_SIZE;
        let key = CacheKey { file_id, page_index };

        // 尝试从缓存读取
        if let Some(page) = self.cache.write().get(&key) {
            self.hits.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            log::trace!("Cache HIT: file={}, offset={}", file_id, offset);
            return Ok(page.clone());
        }

        // 缓存未命中，加载页
        self.misses.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        log::trace!("Cache MISS: file={}, offset={}", file_id, offset);
        
        let page = self.load_page(file_id, page_index)?;
        self.cache.write().put(key, page.clone());
        Ok(page)
    }

    /// 写入页
    pub fn put_page(&self, page: CachePage) {
        let key = CacheKey {
            file_id: page.file_id,
            page_index: page.offset / PAGE_SIZE,
        };
        self.cache.write().put(key, page);
    }

    /// 从磁盘加载页
    fn load_page(&self, file_id: usize, page_index: usize) -> AxResult<CachePage> {
        log::trace!("Loading page: file_id={}, page_index={}", file_id, page_index);
        
        let mut page = CachePage::new(file_id, page_index * PAGE_SIZE);
        let offset = page_index * PAGE_SIZE;
        
        // 通过 axfs 直接读取（临时方案）
        // 实际应该通过文件描述符表获取文件句柄
        // 这里只是占位实现，返回空页
        
        // TODO: 实际实现需要：
        // 1. 维护 file_id -> File 的映射
        // 2. 使用 file.seek(offset) 定位
        // 3. 读取 PAGE_SIZE 字节到 page.data
        
        log::trace!("Page loaded (placeholder): file_id={}, page_index={}", file_id, page_index);
        Ok(page)
    }

    /// 获取缓存命中率
    pub fn hit_rate(&self) -> f32 {
        let hits = self.hits.load(core::sync::atomic::Ordering::Relaxed);
        let misses = self.misses.load(core::sync::atomic::Ordering::Relaxed);
        let total = hits + misses;
        if total == 0 {
            0.0
        } else {
            hits as f32 / total as f32
        }
    }
}
