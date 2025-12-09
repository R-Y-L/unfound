//! Process management for ArceOS.
//!
//! This module provides process management functionality including
//! process creation, forking, waiting, and exit.

#![cfg_attr(not(test), no_std)]
#![feature(doc_cfg)]
#![feature(doc_auto_cfg)]

extern crate alloc;

#[macro_use]
extern crate log;

pub mod process;
pub mod manager;
pub mod fork;
pub mod syscall;

pub use process::{Process, ProcessId, ProcessState};

/// Task extension for process management.
#[derive(Clone, Copy)]
pub struct ProcessTaskExt {
    /// Process ID associated with this task.
    pub process_id: ProcessId,
}

// Define the task extension using axtask's macro
axtask::def_task_ext!(ProcessTaskExt);

/// Initialize process management.
pub fn init() {
    info!("Initialize process management...");
    // Initialize the process manager
    let _ = &*manager::PROCESS_MANAGER;
}
