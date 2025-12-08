/// 进程管理抽象

use alloc::sync::Arc;
use spin::Mutex;
use axerrno::AxResult;

/// 进程控制块
pub struct Process {
    pub pid: usize,
    pub fd_table: Arc<Mutex<FdTable>>,
}

/// 文件描述符表
pub struct FdTable {
    entries: [Option<usize>; 256],
}

impl FdTable {
    pub fn new() -> Self {
        Self {
            entries: [None; 256],
        }
    }

    pub fn alloc_fd(&mut self, file_id: usize) -> AxResult<usize> {
        for (fd, entry) in self.entries.iter_mut().enumerate() {
            if entry.is_none() {
                *entry = Some(file_id);
                return Ok(fd);
            }
        }
        Err(axerrno::AxError::NoMemory)
    }

    pub fn free_fd(&mut self, fd: usize) -> AxResult {
        if fd < self.entries.len() {
            self.entries[fd] = None;
            Ok(())
        } else {
            Err(axerrno::AxError::BadAddress)
        }
    }
}

static CURRENT_PROCESS: Mutex<Option<Arc<Process>>> = Mutex::new(None);

pub fn init() -> AxResult {
    let proc = Arc::new(Process {
        pid: 1,
        fd_table: Arc::new(Mutex::new(FdTable::new())),
    });
    *CURRENT_PROCESS.lock() = Some(proc);
    Ok(())
}

pub fn current_process() -> Arc<Process> {
    CURRENT_PROCESS.lock().as_ref().unwrap().clone()
}
