/// 系统调用处理器
use axhal::trap::{register_trap_handler, TrapFrame};
use axerrno::AxResult;

// 系统调用号定义
const SYS_READ: usize = 63;
const SYS_WRITE: usize = 64;
const SYS_OPEN: usize = 56;
const SYS_CLOSE: usize = 57;
const SYS_OPENAT: usize = 56;
const SYS_FSTAT: usize = 80;
const SYS_EXIT: usize = 93;
const SYS_NOTIFY_ADD_WATCH: usize = 254;  // 自定义系统调用
const SYS_NOTIFY_READ_EVENTS: usize = 255; // 自定义系统调用

/// 系统调用处理函数
#[cfg(feature = "uspace")]
#[register_trap_handler(SYSCALL)]
fn handle_syscall(tf: &mut TrapFrame) -> isize {
    let syscall_num = tf.regs.a7;
    
    match syscall_num {
        SYS_READ => sys_read(
            tf.regs.a0 as usize,
            tf.regs.a1 as *mut u8,
            tf.regs.a2 as usize,
        ),
        SYS_WRITE => sys_write(
            tf.regs.a0 as usize,
            tf.regs.a1 as *const u8,
            tf.regs.a2 as usize,
        ),
        SYS_OPEN | SYS_OPENAT => sys_open(
            tf.regs.a0 as *const u8,
            tf.regs.a1 as u32,
            tf.regs.a2 as u32,
        ),
        SYS_CLOSE => sys_close(tf.regs.a0 as usize),
        SYS_EXIT => sys_exit(tf.regs.a0 as i32),
        SYS_NOTIFY_ADD_WATCH => sys_notify_add_watch(
            tf.regs.a0 as *const u8,
            tf.regs.a1 as u32,
        ),
        SYS_NOTIFY_READ_EVENTS => sys_notify_read_events(
            tf.regs.a0 as *mut u8,
            tf.regs.a1 as usize,
        ),
        _ => {
            warn!("Unknown syscall: {}", syscall_num);
            -1
        }
    }
}

/// sys_read: 从文件描述符读取数据
fn sys_read(fd: usize, buf_ptr: *mut u8, len: usize) -> isize {
    if buf_ptr.is_null() || len == 0 {
        return -1;
    }

    // 构造缓冲区切片（unsafe 操作）
    let buf = unsafe { core::slice::from_raw_parts_mut(buf_ptr, len) };
    
    match uvfs::VfsOps::read(fd, buf) {
        Ok(n) => n as isize,
        Err(e) => {
            warn!("sys_read failed: {:?}", e);
            -1
        }
    }
}

/// sys_write: 向文件描述符写入数据
fn sys_write(fd: usize, buf_ptr: *const u8, len: usize) -> isize {
    if buf_ptr.is_null() || len == 0 {
        return -1;
    }

    let buf = unsafe { core::slice::from_raw_parts(buf_ptr, len) };
    
    match uvfs::VfsOps::write(fd, buf) {
        Ok(n) => n as isize,
        Err(e) => {
            warn!("sys_write failed: {:?}", e);
            -1
        }
    }
}

/// sys_open: 打开文件
fn sys_open(path_ptr: *const u8, flags: u32, mode: u32) -> isize {
    if path_ptr.is_null() {
        return -1;
    }

    // 从指针读取路径字符串
    let path = unsafe {
        let mut len = 0;
        while *path_ptr.add(len) != 0 {
            len += 1;
        }
        let slice = core::slice::from_raw_parts(path_ptr, len);
        core::str::from_utf8_unchecked(slice)
    };

    match uvfs::VfsOps::open(path, flags, mode) {
        Ok(fd) => fd as isize,
        Err(e) => {
            warn!("sys_open failed: {:?}", e);
            -1
        }
    }
}

/// sys_close: 关闭文件描述符
fn sys_close(fd: usize) -> isize {
    match uvfs::VfsOps::close(fd) {
        Ok(_) => 0,
        Err(e) => {
            warn!("sys_close failed: {:?}", e);
            -1
        }
    }
}

/// sys_exit: 退出当前进程
fn sys_exit(exit_code: i32) -> isize {
    info!("Process exit with code: {}", exit_code);
    // TODO: 实际的进程退出逻辑
    0
}

/// sys_notify_add_watch: 添加文件监控
fn sys_notify_add_watch(path_ptr: *const u8, mask: u32) -> isize {
    if path_ptr.is_null() {
        return -1;
    }

    let path = unsafe {
        let mut len = 0;
        while *path_ptr.add(len) != 0 {
            len += 1;
        }
        let slice = core::slice::from_raw_parts(path_ptr, len);
        core::str::from_utf8_unchecked(slice)
    };

    info!("Add watch for path: {}, mask: {}", path, mask);
    // TODO: 实际的监控逻辑
    1 // 返回 watch descriptor
}

/// sys_notify_read_events: 读取文件变化事件
fn sys_notify_read_events(buf_ptr: *mut u8, count: usize) -> isize {
    if buf_ptr.is_null() {
        return -1;
    }

    let watcher = unotify::get_watcher();
    let events = watcher.read_events(count);
    
    info!("Read {} events", events.len());
    events.len() as isize
}

/// 初始化系统调用处理器
pub fn init() {
    info!("Syscall handler initialized");
}
