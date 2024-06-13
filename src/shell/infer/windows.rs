#![cfg(not(unix))]

use crate::shell::Shell;
use log::{debug, warn};
use sysinfo::{ProcessRefreshKind, System, UpdateKind};

pub fn infer_shell() -> Option<Box<dyn Shell>> {
    let mut system = System::new();
    let mut current_pid = sysinfo::get_current_pid().ok();

    system
        .refresh_processes_specifics(ProcessRefreshKind::new().with_exe(UpdateKind::OnlyIfNotSet));

    while let Some(pid) = current_pid {
        if let Some(process) = system.process(pid) {
            current_pid = process.parent();
            debug!("pid {pid} parent process is {current_pid:?}");
            let process_name = process
                .exe()
                .and_then(|x| {
                    tap_none(x.file_stem(), || {
                        warn!("failed to get file stem from {:?}", x);
                    })
                })
                .and_then(|x| {
                    tap_none(x.to_str(), || {
                        warn!("failed to convert file stem to string: {:?}", x);
                    })
                })
                .map(str::to_lowercase);
            if let Some(shell) = process_name
                .as_ref()
                .map(|x| &x[..])
                .and_then(super::shell_from_string)
            {
                return Some(shell);
            }
        } else {
            warn!("process not found for {pid}");
            current_pid = None;
        }
    }

    None
}

fn tap_none<T, F>(opt: Option<T>, f: F) -> Option<T>
where
    F: FnOnce(),
{
    match &opt {
        Some(_) => (),
        None => f(),
    };

    opt
}
