/// 文件相关系统调用实现
use crate::utils;
use ucore::process::current_process;
use uvfs::VfsOps;

/// sys_open - 打开文件
pub fn sys_open(path: &str, flags: u32, mode: u32) -> isize {
    log::debug!("sys_open: path={}, flags={:#x}, mode={:#o}", path, flags, mode);
    
    match VfsOps::open(path, flags, mode) {
        Ok(fd) => fd as isize,
        Err(e) => {
            log::error!("sys_open failed: {:?}", e);
            -(e as i32) as isize
        }
    }
}

/// sys_read - 读取文件
pub fn sys_read(fd: usize, buf: &mut [u8]) -> isize {
    match VfsOps::read(fd, buf) {
        Ok(n) => n as isize,
        Err(e) => -(e as i32) as isize,
    }
}

/// sys_write - 写入文件
pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    match VfsOps::write(fd, buf) {
        Ok(n) => n as isize,
        Err(e) => -(e as i32) as isize,
    }
}

/// sys_close - 关闭文件
pub fn sys_close(fd: usize) -> isize {
    match VfsOps::close(fd) {
        Ok(_) => 0,
        Err(e) => -(e as i32) as isize,
    }
}
