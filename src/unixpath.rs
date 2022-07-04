use anyhow::{Context, Result};
use std::io::ErrorKind;
use std::process::Command;

#[cfg(windows)]
pub fn to_unix_path(path: &str) -> Result<String> {
    // TODO: take parameter to decide between cygpath and wslpath
    let output = Command::new("cygpath")
        .arg(path)
        .output()
        .or_else(|err| match err.kind() {
            // try wslpath if cygpath is not found
            // currently this breaks when WSL is launched from a cygwin-based shell
            ErrorKind::NotFound => Command::new("wsl")
                .arg("-e")
                .arg("wslpath")
                .arg(path)
                .output(),
            _ => Err(err),
        })
        .context("Failed to convert Windows path to Unix")?;

    Ok(std::str::from_utf8(output.stdout.as_slice())?
        .trim()
        .to_string())
}

#[cfg(not(windows))]
pub fn to_unix_path(path: &str) -> Result<String> {
    Ok(path.to_string())
}
