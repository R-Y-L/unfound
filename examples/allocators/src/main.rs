#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

use axalloc::GlobalPage;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    #[cfg(feature = "axstd")]
    println!("[allocators] 内存分配测试开始...");
    
    // 1. 测试小内存分配
    test_small_allocation();
    
    // 2. 测试连续分配
    test_continuous_allocation();
    
    #[cfg(feature = "axstd")]
    println!("[allocators] 所有测试完成!");
}

fn test_small_allocation() {
    #[cfg(feature = "axstd")]
    println!("[allocators] 测试1: 分配1个页面");
    
    // 使用GlobalPage分配内存
    let page = match GlobalPage::alloc() {
        Ok(p) => p,
        Err(_) => {
            #[cfg(feature = "axstd")]
            println!("[allocators] 分配失败!");
            return;
        }
    };
    
    #[cfg(feature = "axstd")]
    println!("[allocators] 分配成功，地址: {:p}", page.start_vaddr().as_ptr());
    
    // 释放内存（通过Drop自动释放）
    drop(page);
    
    #[cfg(feature = "axstd")]
    println!("[allocators] 内存释放成功");
}

fn test_continuous_allocation() {
    #[cfg(feature = "axstd")]
    println!("[allocators] 测试2: 分配3个连续页面");
    
    // 分配3个连续页面
    let page = match GlobalPage::alloc_contiguous(3, 4096) {
        Ok(p) => p,
        Err(_) => {
            #[cfg(feature = "axstd")]
            println!("[allocators] 分配失败!");
            return;
        }
    };
    
    #[cfg(feature = "axstd")]
    println!("[allocators] 分配成功，地址: {:p}", page.start_vaddr().as_ptr());
    
    // 释放内存（通过Drop自动释放）
    drop(page);
    
    #[cfg(feature = "axstd")]
    println!("[allocators] 内存释放成功");
}