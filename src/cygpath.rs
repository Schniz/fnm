use std::process::Command;
use anyhow::{Context, Result};

#[cfg(windows)]
pub fn cygpath(path: &str) -> Result<String> {
    let output = Command::new("cygpath")
        .arg(path)
        .output()
        .context("Failed to convert Windows path to Unix")?;

    Ok(std::str::from_utf8(output.stdout.as_slice())?.trim().to_string())
}

#[cfg(not(windows))]
pub fn cygpath(path: &str) -> Result<String> {
    Ok(path.to_string())
}
