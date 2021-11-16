use super::{Bash, Fish, PowerShell, Shell, WindowsCmd, Zsh};
use log::debug;
use std::ffi::OsStr;
use sysinfo::{ProcessExt, System, SystemExt};

#[derive(Debug)]
struct ProcessInfo {
    parent_pid: Option<u32>,
    command: String,
}

pub fn infer_shell() -> Option<Box<dyn Shell>> {
    let system = System::new_all();
    let mut current_pid = sysinfo::get_current_pid().ok();

    while let Some(pid) = current_pid {
        if let Some(process) = system.process(pid) {
            current_pid = process.parent();
            let process_name = process
                .exe()
                .file_stem()
                .and_then(OsStr::to_str)
                .map(str::to_lowercase);
            let sliced = process_name.as_ref().map(|x| &x[..]);
            match sliced {
                Some("sh" | "bash") => return Some(Box::from(Bash)),
                Some("zsh") => return Some(Box::from(Zsh)),
                Some("fish") => return Some(Box::from(Fish)),
                Some("pwsh" | "powershell") => return Some(Box::from(PowerShell)),
                Some("cmd") => return Some(Box::from(WindowsCmd)),
                cmd_name => debug!("binary is not a supported shell: {:?}", cmd_name),
            };
        } else {
            current_pid = None;
        }
    }

    None
}
