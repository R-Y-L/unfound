/// ARC (Adaptive Replacement Cache) 缓存算法实现
/// 
/// ARC 是一种自适应缓存替换算法，综合考虑最近性(Recency)和频繁性(Frequency)
/// 
/// 核心思想：
/// - T1: 最近访问一次的页面 (Recency)
/// - T2: 最近访问多次的页面 (Frequency)
/// - B1: T1 的幽灵列表 (被淘汰但记录历史)
/// - B2: T2 的幽灵列表
/// - p: 自适应分割点，动态调整 T1 和 T2 的大小

use alloc::collections::VecDeque;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use spin::RwLock;
use core::sync::atomic::{AtomicUsize, Ordering};

/// 缓存项
#[derive(Clone, Debug)]
pub struct CacheEntry<V> {
    pub value: V,
    pub dirty: bool,
}

/// ARC 缓存主结构
pub struct ARCache<K: Ord + Clone, V: Clone> {
    /// T1: 最近访问一次 (短期热点)
    t1: RwLock<VecDeque<K>>,
    /// T2: 频繁访问 (长期热点)
    t2: RwLock<VecDeque<K>>,
    /// B1: T1 幽灵列表 (记录被淘汰的 T1 项)
    b1: RwLock<VecDeque<K>>,
    /// B2: T2 幽灵列表 (记录被淘汰的 T2 项)
    b2: RwLock<VecDeque<K>>,
    
    /// 实际存储数据 (T1 + T2)
    cache: RwLock<BTreeMap<K, CacheEntry<V>>>,
    
    /// 自适应分割点：T1 的目标大小
    p: AtomicUsize,
    
    /// 总容量 c
    capacity: usize,
    
    /// 统计信息
    hits: AtomicUsize,
    misses: AtomicUsize,
}

impl<K: Ord + Clone, V: Clone> ARCache<K, V> {
    /// 创建新的 ARC 缓存
    pub fn new(capacity: usize) -> Self {
        Self {
            t1: RwLock::new(VecDeque::new()),
            t2: RwLock::new(VecDeque::new()),
            b1: RwLock::new(VecDeque::new()),
            b2: RwLock::new(VecDeque::new()),
            cache: RwLock::new(BTreeMap::new()),
            p: AtomicUsize::new(0),
            capacity,
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
        }
    }

    /// 获取缓存项
    pub fn get(&self, key: &K) -> Option<V> {
        let cache = self.cache.read();
        
        if let Some(entry) = cache.get(key) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            
            // 命中后需要移动到 T2 (提升为频繁访问)
            drop(cache);
            self.promote_to_t2(key);
            
            self.cache.read().get(key).map(|e| e.value.clone())
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// 插入或更新缓存项
    pub fn put(&self, key: K, value: V) {
        let mut cache = self.cache.write();
        
        // Case 1: 已经在缓存中 (T1 或 T2)
        if cache.contains_key(&key) {
            cache.insert(key.clone(), CacheEntry { value, dirty: false });
            drop(cache);
            self.promote_to_t2(&key);
            return;
        }

        drop(cache);

        // Case 2: 在 B1 中 (曾经在 T1，被淘汰了)
        if self.in_b1(&key) {
            self.handle_b1_hit(&key, value);
            return;
        }

        // Case 3: 在 B2 中 (曾经在 T2，被淘汰了)
        if self.in_b2(&key) {
            self.handle_b2_hit(&key, value);
            return;
        }

        // Case 4: 全新的键
        self.insert_new(key, value);
    }

    /// 检查是否在 B1
    fn in_b1(&self, key: &K) -> bool {
        self.b1.read().contains(key)
    }

    /// 检查是否在 B2
    fn in_b2(&self, key: &K) -> bool {
        self.b2.read().contains(key)
    }

    /// 处理 B1 命中：增加 p (给 T1 更多空间)
    fn handle_b1_hit(&self, key: &K, value: V) {
        // 调整 p: 增加 T1 的目标大小
        let b1_len = self.b1.read().len();
        let b2_len = self.b2.read().len();
        let delta = if b1_len >= b2_len { 1 } else { b2_len / b1_len };
        
        let p = self.p.load(Ordering::Relaxed);
        self.p.store((p + delta).min(self.capacity), Ordering::Relaxed);

        // 从 B1 移除
        self.b1.write().retain(|k| k != key);

        // 替换并插入到 T2 (因为是二次访问)
        self.replace(key);
        self.cache.write().insert(key.clone(), CacheEntry { value, dirty: false });
        self.t2.write().push_back(key.clone());
    }

    /// 处理 B2 命中：减少 p (给 T2 更多空间)
    fn handle_b2_hit(&self, key: &K, value: V) {
        // 调整 p: 减少 T1 的目标大小
        let b1_len = self.b1.read().len();
        let b2_len = self.b2.read().len();
        let delta = if b2_len >= b1_len { 1 } else { b1_len / b2_len };
        
        let p = self.p.load(Ordering::Relaxed);
        self.p.store(p.saturating_sub(delta), Ordering::Relaxed);

        // 从 B2 移除
        self.b2.write().retain(|k| k != key);

        // 替换并插入到 T2
        self.replace(key);
        self.cache.write().insert(key.clone(), CacheEntry { value, dirty: false });
        self.t2.write().push_back(key.clone());
    }

    /// 插入全新的键
    fn insert_new(&self, key: K, value: V) {
        let t1_len = self.t1.read().len();
        let t2_len = self.t2.read().len();
        let b1_len = self.b1.read().len();
        let l1_len = t1_len + b1_len;

        // 如果 L1 (T1 + B1) 达到容量
        if l1_len == self.capacity {
            if t1_len < self.capacity {
                // B1 有内容，删除 B1 最老的
                self.b1.write().pop_front();
                self.replace(&key);
            } else {
                // T1 满了，删除 T1 最老的
                if let Some(old_key) = self.t1.write().pop_front() {
                    self.cache.write().remove(&old_key);
                }
            }
        } else {
            // L1 + L2 达到 2c，需要删除
            let total = t1_len + t2_len + b1_len + self.b2.read().len();
            if total >= 2 * self.capacity {
                if total == 2 * self.capacity {
                    // 删除 B2 最老的
                    self.b2.write().pop_front();
                }
                self.replace(&key);
            }
        }

        // 插入到 T1 (首次访问)
        self.cache.write().insert(key.clone(), CacheEntry { value, dirty: false });
        self.t1.write().push_back(key);
    }

    /// 替换算法核心：根据 p 决定从 T1 还是 T2 淘汰
    fn replace(&self, key: &K) {
        let t1_len = self.t1.read().len();
        let p = self.p.load(Ordering::Relaxed);

        let should_evict_from_t1 = if t1_len > 0 {
            t1_len > p || (self.in_b2(key) && t1_len == p)
        } else {
            false
        };

        if should_evict_from_t1 {
            // 从 T1 淘汰到 B1
            if let Some(old_key) = self.t1.write().pop_front() {
                self.cache.write().remove(&old_key);
                
                // 加入 B1 (保留历史)
                let mut b1 = self.b1.write();
                b1.push_back(old_key);
                
                // B1 也有大小限制
                if b1.len() > self.capacity {
                    b1.pop_front();
                }
            }
        } else {
            // 从 T2 淘汰到 B2
            if let Some(old_key) = self.t2.write().pop_front() {
                self.cache.write().remove(&old_key);
                
                // 加入 B2 (保留历史)
                let mut b2 = self.b2.write();
                b2.push_back(old_key);
                
                // B2 也有大小限制
                if b2.len() > self.capacity {
                    b2.pop_front();
                }
            }
        }
    }

    /// 将页面从 T1 提升到 T2
    fn promote_to_t2(&self, key: &K) {
        // 从 T1 移除
        let was_in_t1 = {
            let mut t1 = self.t1.write();
            let pos = t1.iter().position(|k| k == key);
            if let Some(pos) = pos {
                t1.remove(pos);
                true
            } else {
                false
            }
        };

        if was_in_t1 {
            // 移动到 T2
            self.t2.write().push_back(key.clone());
        } else {
            // 已经在 T2，移到末尾 (最近使用)
            let mut t2 = self.t2.write();
            if let Some(pos) = t2.iter().position(|k| k == key) {
                t2.remove(pos);
                t2.push_back(key.clone());
            }
        }
    }

    /// 使缓存项无效
    pub fn invalidate(&self, key: &K) {
        self.cache.write().remove(key);
        self.t1.write().retain(|k| k != key);
        self.t2.write().retain(|k| k != key);
        self.b1.write().retain(|k| k != key);
        self.b2.write().retain(|k| k != key);
    }

    /// 获取缓存命中率
    pub fn hit_rate(&self) -> f64 {
        let hits = self.hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
        let total = hits + misses;
        
        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
        }
    }

    /// 获取缓存统计信息
    pub fn stats(&self) -> ARCStats {
        ARCStats {
            t1_size: self.t1.read().len(),
            t2_size: self.t2.read().len(),
            b1_size: self.b1.read().len(),
            b2_size: self.b2.read().len(),
            p: self.p.load(Ordering::Relaxed),
            capacity: self.capacity,
            hits: self.hits.load(Ordering::Relaxed),
            misses: self.misses.load(Ordering::Relaxed),
        }
    }
}

/// ARC 统计信息
#[derive(Debug, Clone)]
pub struct ARCStats {
    pub t1_size: usize,      // 最近访问一次的数量
    pub t2_size: usize,      // 频繁访问的数量
    pub b1_size: usize,      // B1 幽灵列表大小
    pub b2_size: usize,      // B2 幽灵列表大小
    pub p: usize,            // 当前分割点
    pub capacity: usize,     // 总容量
    pub hits: usize,         // 命中次数
    pub misses: usize,       // 未命中次数
}

impl ARCStats {
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}
