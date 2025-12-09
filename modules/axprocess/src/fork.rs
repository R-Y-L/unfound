use axtask::{TaskInner, AxTaskRefExt};
use crate::manager::PROCESS_MANAGER;
use crate::process::ProcessId;
use crate::ProcessTaskExt;

/// Fork the current process.
///
/// # Safety
///
/// This function is unsafe because it manipulates process memory.
pub unsafe fn fork() -> i32 {
    let current = axtask::current();

    let task_ext = current
        .as_task_ref()
        .task_ext_ref::<ProcessTaskExt>()
        .expect("Failed to get task extension");

    let mut pm = PROCESS_MANAGER.lock();
    let parent_pid = task_ext.process_id.0;

    // Get parent process to copy resources from
    let parent_process = match pm.get_process(parent_pid) {
        Some(p) => p,
        None => return -1,
    };

    // Clone parent's address space and namespace for child
    let child_aspace = parent_process.aspace().clone();
    let child_namespace = parent_process.namespace().clone();

    // Create a new process for the child
    let child_pid = match pm.create_process(
        "child".into(),
        parent_pid,
        child_aspace,
        child_namespace,
    ) {
        Ok(pid) => pid.0,
        Err(_) => return -1,
    };

    // In the parent process, return the child PID
    // In the child process, this would return 0
    // For now, we just return the child PID as we're not actually forking
    // Full fork implementation would require more complex task duplication
    if let Some(_child_process) = pm.get_process(child_pid) {
        // Spawn a new task for the child process
        let mut child_task = TaskInner::new(|| {}, "child-task".into(), axconfig::TASK_STACK_SIZE);
        
        // Initialize task extension for the child task
        child_task.init_task_ext(ProcessTaskExt {
            process_id: ProcessId(child_pid),
        });
        
        // Spawn the task
        let _child_task_ref = axtask::spawn_task(child_task);
    }

    child_pid as i32
}

/// Wait for a child process to exit.
pub fn wait(wstatus: *mut i32) -> i32 {
    let current = axtask::current();
    
    let task_ext = current
        .as_task_ref()
        .task_ext_ref::<ProcessTaskExt>()
        .expect("Failed to get task extension");

    let pm = PROCESS_MANAGER.lock();
    let current_pid = task_ext.process_id.0;

    // Find a child process that has exited
    // This is a simplified implementation
    for process in pm.all_processes() {
        if process.ppid().0 == current_pid {
            // Wait for the child process to exit
            process.wait_queue().wait();
            
            let exit_code = process.exit_code();
            if !wstatus.is_null() {
                unsafe {
                    *wstatus = exit_code;
                }
            }
            return process.pid().0 as i32;
        }
    }

    -1 // No child process found
}
