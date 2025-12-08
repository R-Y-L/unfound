#![no_std]
#![no_main]

#[macro_use]
extern crate axlog;

mod syscall;

use core::panic::PanicInfo;

/// 内核入口函数
#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb: usize) {
    axlog::init("info");
    info!("Starting Unfound OS...");

    // 初始化核心模块
    ucore::init().expect("Failed to initialize ucore");
    
    // 初始化文件系统
    uvfs::init().expect("Failed to initialize uvfs");
    
    // 初始化创新模块
    ucache::init(256).expect("Failed to initialize ucache"); // 256页缓存
    unotify::init().expect("Failed to initialize unotify");
    
    // 初始化系统调用层
    syscall::init();

    info!("Unfound OS initialized successfully!");
    info!("UCache capacity: 256 pages (1MB)");
    info!("UNotify max events: 1024");

    // TODO: 启动用户态程序
    
    info!("System halted.");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    arch_boot::panic(info)
}
