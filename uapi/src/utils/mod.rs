/// 工具函数模块

pub fn validate_path(path: &str) -> bool {
    !path.is_empty() && path.len() < 4096
}

pub fn normalize_flags(flags: u32) -> u32 {
    flags & 0xFFFF  // 屏蔽无效位
}
