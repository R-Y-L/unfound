#![no_std]

extern crate alloc;

pub mod syscall;
pub mod interface;
pub mod utils;

use axerrno::AxResult;

/// 系统调用初始化
pub fn init() {
    log::info!("Initializing unfound UAPI...");
}

/// 系统调用错误码转换
pub fn to_errno(result: AxResult<usize>) -> isize {
    match result {
        Ok(v) => v as isize,
        Err(e) => -(e as i32) as isize,
    }
}
