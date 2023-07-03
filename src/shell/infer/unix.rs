#![cfg(unix)]

use crate::shell::Shell;
use log::debug;
use std::io::{Error, ErrorKind};
use thiserror::Error;

#[derive(Debug)]
struct ProcessInfo {
    parent_pid: Option<u32>,
    command: String,
}

const MAX_ITERATIONS: u8 = 10;

pub fn infer_shell() -> Option<Box<dyn Shell>> {
    let mut pid = Some(std::process::id());
    let mut visited = 0;

    while let Some(current_pid) = pid {
        if visited > MAX_ITERATIONS {
            return None;
        }

        let process_info = get_process_info(current_pid)
            .map_err(|err| {
                debug!("{}", err);
                err
            })
            .ok()?;
        let binary = process_info
            .command
            .trim_start_matches('-')
            .split('/')
            .last()?;

        if let Some(shell) = super::shell_from_string(binary) {
            return Some(shell);
        }

        pid = process_info.parent_pid;
        visited += 1;
    }

    None
}

fn get_process_info(pid: u32) -> Result<ProcessInfo, ProcessInfoError> {
    use std::io::{BufRead, BufReader};
    use std::process::Command;

    let buffer = Command::new("ps")
        .arg("-o")
        .arg("ppid,comm")
        .arg(pid.to_string())
        .stdout(std::process::Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::from(ErrorKind::UnexpectedEof))?;

    let mut lines = BufReader::new(buffer).lines();

    // skip header line
    lines
        .next()
        .ok_or_else(|| Error::from(ErrorKind::UnexpectedEof))??;

    let line = lines
        .next()
        .ok_or_else(|| Error::from(ErrorKind::NotFound))??;

    let mut parts = line.split_whitespace();
    let ppid = parts.next().ok_or_else(|| ProcessInfoError::Parse {
        expectation: "Can't read the ppid from ps, should be the first item in the table",
        got: line.to_string(),
    })?;
    let command = parts.next().ok_or_else(|| ProcessInfoError::Parse {
        expectation: "Can't read the command from ps, should be the second item in the table",
        got: line.to_string(),
    })?;

    Ok(ProcessInfo {
        parent_pid: ppid.parse().ok(),
        command: command.into(),
    })
}

#[derive(Debug, Error)]
enum ProcessInfoError {
    #[error("Can't read process info: {source}")]
    Io {
        #[source]
        #[from]
        source: std::io::Error,
    },
    #[error("Can't parse process info output. {expectation}. Got: {got}")]
    Parse {
        got: String,
        expectation: &'static str,
    },
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::process::{Command, Stdio};

    #[test]
    fn test_get_process_info() -> anyhow::Result<()> {
        let subprocess = Command::new("bash")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let process_info = get_process_info(subprocess.id());
        let parent_pid = process_info.ok().and_then(|x| x.parent_pid);
        assert_eq!(parent_pid, Some(std::process::id()));
        Ok(())
    }
}
