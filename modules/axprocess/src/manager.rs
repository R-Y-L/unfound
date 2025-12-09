use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicU32, Ordering};

use axsync::Mutex;
use axmm::AddrSpace;
use axns::AxNamespace;

use crate::process::{Process, ProcessId};

pub struct ProcessManager {
    processes: BTreeMap<u32, Arc<Process>>,
    /// Next process ID.
    next_pid: AtomicU32,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            next_pid: AtomicU32::new(1),
        }
    }

    /// Allocate a new process ID.
    pub fn alloc_pid(&self) -> u32 {
        self.next_pid.fetch_add(1, Ordering::SeqCst)
    }

    /// Create a new process.
    pub fn create_process(
        &mut self,
        name: String,
        ppid: u32,
        aspace: Arc<AddrSpace>,
        namespace: Arc<AxNamespace>,
    ) -> Result<ProcessId, &'static str> {
        let pid = self.alloc_pid();

        let process = Process::new(
            ProcessId(pid),
            ProcessId(ppid),
            name,
            aspace,
            namespace,
        );

        self.processes.insert(pid, process);
        Ok(ProcessId(pid))
    }

    /// Get a process by its ID.
    pub fn get_process(&self, pid: u32) -> Option<Arc<Process>> {
        self.processes.get(&pid).cloned()
    }

    /// Remove a process by its ID.
    pub fn remove_process(&mut self, pid: u32) -> Option<Arc<Process>> {
        self.processes.remove(&pid)
    }

    /// Get all processes.
    pub fn all_processes(&self) -> alloc::vec::Vec<Arc<Process>> {
        self.processes.values().cloned().collect()
    }
}

lazy_static::lazy_static! {
    pub static ref PROCESS_MANAGER: Mutex<ProcessManager> = Mutex::new(ProcessManager::new());
}
