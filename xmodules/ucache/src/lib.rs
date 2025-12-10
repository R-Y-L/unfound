#![no_std]
//! UCache - 智能文件缓存模块
//! 
//! 创新特性：
//! - ARC (Adaptive Replacement Cache) 算法
//! - 自适应预读（Sequential/Random模式检测）
//! - 缓存统计与监控

extern crate alloc;

mod arc_cache;

pub use arc_cache::{ARCache, ARCStats, CacheEntry};

use axerrno::AxResult;
use spin::Mutex;
use alloc::sync::Arc;
use alloc::string::String;
use alloc::vec::Vec;

/// 文件缓存类型 (使用 ARC 算法)
pub type UCache = ARCache<String, Vec<u8>>;

/// 全局文件缓存实例
static GLOBAL_CACHE: Mutex<Option<Arc<UCache>>> = Mutex::new(None);

/// 初始化文件缓存
pub fn init(capacity: usize) -> AxResult {
    log::info!("[UCache] Initializing with ARC algorithm, capacity: {} entries", capacity);
    let cache = Arc::new(ARCache::new(capacity));
    *GLOBAL_CACHE.lock() = Some(cache);
    Ok(())
}

/// 获取全局缓存实例
pub fn get_cache() -> Option<Arc<UCache>> {
    GLOBAL_CACHE.lock().clone()
}
