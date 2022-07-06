#[cfg(windows)]
/// Returns whether the windows executable is being run from WSL.
pub fn is_wsl() -> bool {
    use std::ffi::OsStr;
    use sysinfo::{ProcessExt, System, SystemExt};

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

            if let Some(name) = process_name {
                if name == "wsl" {
                    return true;
                }
            }
        } else {
            current_pid = None;
        }
    }

    false
}

#[cfg(unix)]
/// Not a Windows executable, so return false.
/// A Linux executable run in WSL doesn't need to behave differently.
pub fn is_wsl() -> bool {
    false
}
