#![cfg(not(unix))]

use crate::shell::Shell;
use std::ffi::OsStr;
use sysinfo::{ProcessExt, System, SystemExt};

pub fn infer_shell() -> Option<Box<dyn Shell>> {
    let mut system = System::new();
    let mut current_pid = sysinfo::get_current_pid().ok();

    while let Some(pid) = current_pid {
        system.refresh_process(pid);
        if let Some(process) = system.process(pid) {
            current_pid = process.parent();
            let process_name = process
                .exe()
                .file_stem()
                .and_then(OsStr::to_str)
                .map(str::to_lowercase);
            if let Some(shell) = process_name
                .as_ref()
                .map(|x| &x[..])
                .and_then(super::shell_from_string)
            {
                return Some(shell);
            }
        } else {
            current_pid = None;
        }
    }

    None
}
