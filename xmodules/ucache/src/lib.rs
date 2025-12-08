#![no_std]
//! UCache - 智能页缓存模块
//! 
//! 创新特性：
//! - LRU淘汰策略
//! - 自适应预读（Sequential/Random模式检测）
//! - 缓存统计与监控

extern crate alloc;

mod page_cache;
mod readahead;
mod stats;

pub use page_cache::{PageCache, CachePage};
pub use readahead::{ReadaheadPolicy, AccessPattern};
pub use stats::CacheStats;

use axerrno::AxResult;
use spin::Mutex;
use alloc::sync::Arc;

/// 全局页缓存实例
static GLOBAL_PAGE_CACHE: Mutex<Option<Arc<PageCache>>> = Mutex::new(None);

/// 初始化页缓存
pub fn init(capacity: usize) -> AxResult {
    log::info!("Initializing UCache with capacity: {} pages", capacity);
    let cache = Arc::new(PageCache::new(capacity));
    *GLOBAL_PAGE_CACHE.lock() = Some(cache);
    Ok(())
}

/// 获取全局页缓存实例
pub fn get_cache() -> Arc<PageCache> {
    GLOBAL_PAGE_CACHE.lock().as_ref().unwrap().clone()
}
