/// 自适应预读策略

/// 访问模式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccessPattern {
    Sequential,  // 顺序访问
    Random,      // 随机访问
    Unknown,     // 未知模式
}

/// 预读策略
pub struct ReadaheadPolicy {
    pattern: AccessPattern,
    last_offset: usize,
    sequential_count: usize,
}

impl ReadaheadPolicy {
    pub fn new() -> Self {
        Self {
            pattern: AccessPattern::Unknown,
            last_offset: 0,
            sequential_count: 0,
        }
    }

    /// 更新访问模式
    pub fn update(&mut self, offset: usize) {
        if offset == self.last_offset + 4096 {
            // 顺序访问
            self.sequential_count += 1;
            if self.sequential_count > 3 {
                self.pattern = AccessPattern::Sequential;
            }
        } else {
            // 随机访问
            self.sequential_count = 0;
            self.pattern = AccessPattern::Random;
        }
        self.last_offset = offset;
    }

    /// 计算预读窗口大小
    pub fn readahead_size(&self) -> usize {
        match self.pattern {
            AccessPattern::Sequential => 8,  // 预读8页
            AccessPattern::Random => 1,      // 仅读1页
            AccessPattern::Unknown => 2,     // 默认2页
        }
    }
}
