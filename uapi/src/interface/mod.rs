/// 用户态系统调用接口绑定层
/// 
/// 将内核实现的系统调用暴露给用户态，提供符合POSIX标准的接口

use crate::syscall;

#[no_mangle]
pub extern "C" fn open(path: *const u8, flags: u32, mode: u32) -> isize {
    let path_str = unsafe {
        core::str::from_utf8_unchecked(core::slice::from_raw_parts(path, 256))
    };
    syscall::sys_open(path_str, flags, mode)
}

#[no_mangle]
pub extern "C" fn read(fd: usize, buf: *mut u8, count: usize) -> isize {
    let buf_slice = unsafe { core::slice::from_raw_parts_mut(buf, count) };
    syscall::sys_read(fd, buf_slice)
}

#[no_mangle]
pub extern "C" fn write(fd: usize, buf: *const u8, count: usize) -> isize {
    let buf_slice = unsafe { core::slice::from_raw_parts(buf, count) };
    syscall::sys_write(fd, buf_slice)
}

#[no_mangle]
pub extern "C" fn close(fd: usize) -> isize {
    syscall::sys_close(fd)
}
