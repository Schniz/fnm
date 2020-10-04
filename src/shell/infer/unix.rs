#![cfg(unix)]

use super::super::{Bash, Fish, PowerShell, Shell, Zsh};
use std::io::{Error, ErrorKind};

#[derive(Debug)]
struct ProcessInfo {
    parent_pid: Option<u32>,
    command: String,
}

const MAX_ITERATIONS: u8 = 10;

pub fn infer_shell() -> Option<Box<dyn Shell>> {
    let mut pid = Some(std::process::id());
    let mut visited = 0;

    while pid != None && visited < MAX_ITERATIONS {
        let process_info = get_process_info(pid.unwrap()).ok()?;

        match process_info.command.as_str().trim_start_matches('-') {
            "sh" | "bash" => return Some(Box::from(Bash)),
            "zsh" => return Some(Box::from(Zsh)),
            "fish" => return Some(Box::from(Fish)),
            "pwsh" => return Some(Box::from(PowerShell)),
            _ => (),
        };

        pid = process_info.parent_pid;

        visited = visited + 1;
    }

    None
}

fn get_process_info(pid: u32) -> std::io::Result<ProcessInfo> {
    use std::io::{BufRead, BufReader};
    use std::process::Command;

    let buffer = Command::new("ps")
        .arg("-o")
        .arg("ppid,comm")
        .arg(pid.to_string())
        .stdout(std::process::Stdio::piped())
        .spawn()?
        .stdout
        .ok_or(Error::from(ErrorKind::UnexpectedEof))?;

    let mut lines = BufReader::new(buffer).lines();

    // skip header line
    lines
        .next()
        .ok_or(Error::from(ErrorKind::UnexpectedEof))??;

    let line = lines.next().ok_or(Error::from(ErrorKind::NotFound))??;

    let mut parts = line.trim().split_whitespace();
    let ppid = parts
        .next()
        .expect("Can't read the ppid from ps, should be the first item in the table");
    let command = parts
        .next()
        .expect("Can't read the command from ps, should be the first item in the table");

    Ok(ProcessInfo {
        parent_pid: u32::from_str_radix(ppid, 10).ok(),
        command: command.into(),
    })
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::process::{Command, Stdio};

    #[test]
    fn test_get_process_info() {
        let subprocess = Command::new("bash")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Can't execute command");
        let process_info = get_process_info(subprocess.id());
        let parent_pid = process_info.ok().and_then(|x| x.parent_pid);
        assert_eq!(parent_pid, Some(std::process::id()));
    }
}
