#![no_std]

extern crate alloc;

pub mod process;
pub mod memory;

use axerrno::AxResult;

/// 核心抽象层初始化
pub fn init() -> AxResult {
    log::info!("Initializing unfound core abstractions...");
    process::init()?;
    Ok(())
}
