#![cfg(windows)]

use super::super::{Bash, PowerShell, Shell, WindowsCmd};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ProcessInfo {
    #[serde(rename = "ExecutablePath")]
    executable_path: Option<std::path::PathBuf>,
    #[serde(rename = "ParentProcessId")]
    parent_pid: u32,
    #[serde(rename = "ProcessId")]
    pid: u32,
}

pub fn infer_shell() -> Option<Box<dyn Shell>> {
    let process_map = get_process_map().ok()?;
    let process_tree = get_process_tree(process_map, std::process::id());

    for process in process_tree {
        if let Some(exec_path) = process.executable_path {
            match exec_path.file_name().and_then(|x| x.to_str()) {
                Some("cmd.exe") => {
                    return Some(Box::from(WindowsCmd));
                }
                Some("bash.exe") => {
                    return Some(Box::from(Bash));
                }
                Some("powershell.exe") | Some("pwsh.exe") => {
                    return Some(Box::from(PowerShell));
                }
                _ => {}
            }
        }
    }

    None
}

type ProcessMap = HashMap<u32, ProcessInfo>;

pub fn get_process_tree(mut process_map: ProcessMap, pid: u32) -> Vec<ProcessInfo> {
    let mut vec = vec![];
    let mut current = process_map.remove(&pid);

    while let Some(process) = current {
        current = process_map.remove(&process.parent_pid);
        vec.push(process);
    }

    vec
}

pub fn get_process_map() -> std::io::Result<ProcessMap> {
    let stdout = std::process::Command::new("wmic")
        .args(&[
            "process",
            "get",
            "processid,parentprocessid,executablepath",
            "/format:csv",
        ])
        .stdout(std::process::Stdio::piped())
        .spawn()?
        .stdout
        .ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;

    let mut reader = csv::Reader::from_reader(stdout);
    let hashmap: HashMap<_, _> = reader
        .deserialize::<ProcessInfo>()
        .filter_map(Result::ok)
        .map(|x| (x.pid, x))
        .collect();
    Ok(hashmap)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_me() {
        let processes = super::get_process_map().unwrap();
        assert!(processes.contains_key(&std::process::id()));
    }
}
