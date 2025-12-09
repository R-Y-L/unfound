/// Syscall implementations for process management.

use axtask::AxTaskRefExt;
use crate::fork::{fork, wait};
use crate::manager::PROCESS_MANAGER;
use crate::ProcessTaskExt;

/// Fork syscall implementation.
pub extern "C" fn syscall_fork() -> i32 {
    unsafe { fork() }
}

/// Wait syscall implementation.
pub extern "C" fn syscall_wait(wstatus: *mut i32) -> i32 {
    wait(wstatus)
}

/// Exit syscall implementation.
pub extern "C" fn syscall_exit(code: i32) -> ! {
    let current = axtask::current();
    if let Ok(task_ext) = current.as_task_ref().task_ext_ref::<ProcessTaskExt>() {
        // Update process state
        let pm = PROCESS_MANAGER.lock();
        if let Some(process) = pm.get_process(task_ext.process_id.0) {
            process.set_exit_code(code);
            process.wait_queue().notify_all(true);
        }
    }
    axtask::exit(code)
}

/// Get process ID syscall implementation.
pub extern "C" fn syscall_getpid() -> i32 {
    let current = axtask::current();
    if let Ok(task_ext) = current.as_task_ref().task_ext_ref::<ProcessTaskExt>() {
        return task_ext.process_id.0 as i32;
    }
    -1
}

/// Get parent process ID syscall implementation.
pub extern "C" fn syscall_getppid() -> i32 {
    let current = axtask::current();
    if let Ok(task_ext) = current.as_task_ref().task_ext_ref::<ProcessTaskExt>() {
        let pm = PROCESS_MANAGER.lock();
        if let Some(process) = pm.get_process(task_ext.process_id.0) {
            return process.ppid().0 as i32;
        }
    }
    -1
}
