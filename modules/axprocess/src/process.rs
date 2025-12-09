use alloc::string::String;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicI32, AtomicU8, Ordering};

use axmm::AddrSpace;
use axns::AxNamespace;
use axtask::WaitQueue;

/// A unique identifier for a process.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ProcessId(pub u32);

/// The possible states of a process.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ProcessState {
    /// Process is running.
    Running = 1,
    /// Process is exited.
    Exited = 2,
}

impl From<u8> for ProcessState {
    fn from(state: u8) -> Self {
        match state {
            1 => Self::Running,
            2 => Self::Exited,
            _ => Self::Running,
        }
    }
}

/// The inner process structure.
pub struct Process {
    /// Process ID.
    pid: ProcessId,
    /// Parent process ID.
    ppid: ProcessId,
    /// Process name.
    name: String,
    /// Address space.
    aspace: Arc<AddrSpace>,
    /// Namespace.
    namespace: Arc<AxNamespace>,
    /// Process state.
    state: AtomicU8,
    /// Exit code.
    exit_code: AtomicI32,
    /// Wait queue for process exit.
    wait_queue: axtask::WaitQueue,
}

impl Process {
    /// Create a new process.
    pub fn new(
        pid: ProcessId,
        ppid: ProcessId,
        name: String,
        aspace: Arc<AddrSpace>,
        namespace: Arc<AxNamespace>,
    ) -> Arc<Self> {
        Arc::new(Self {
            pid,
            ppid,
            name,
            aspace,
            namespace,
            state: AtomicU8::new(ProcessState::Running as u8),
            exit_code: AtomicI32::new(0),
            wait_queue: axtask::WaitQueue::new(),
        })
    }

    /// Get process ID.
    pub fn pid(&self) -> ProcessId {
        self.pid
    }

    /// Get parent process ID.
    pub fn ppid(&self) -> ProcessId {
        self.ppid
    }

    /// Get process name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get address space.
    pub fn aspace(&self) -> &Arc<AddrSpace> {
        &self.aspace
    }

    /// Get namespace.
    pub fn namespace(&self) -> &Arc<AxNamespace> {
        &self.namespace
    }

    /// Get process state.
    pub fn state(&self) -> ProcessState {
        self.state.load(Ordering::Acquire).into()
    }

    /// Set process state.
    pub fn set_state(&self, state: ProcessState) {
        self.state.store(state as u8, Ordering::Release);
    }

    /// Get exit code.
    pub fn exit_code(&self) -> i32 {
        self.exit_code.load(Ordering::Acquire)
    }

    /// Set exit code.
    pub fn set_exit_code(&self, code: i32) {
        self.exit_code.store(code, Ordering::Release);
    }

    /// Get wait queue.
    pub fn wait_queue(&self) -> &WaitQueue {
        &self.wait_queue
    }
}
