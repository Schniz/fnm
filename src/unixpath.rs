use crate::shell::PathFormatter;
use anyhow::Result;
use std::path::Path;
use crate::wsl::is_wsl;

#[cfg(windows)]
pub fn to_unix_path(path: &Path, formatter: &PathFormatter) -> Result<String> {
    use anyhow::anyhow;
    use std::process::Command;

    if let PathFormatter::None = formatter {
        return Ok(path.to_str().unwrap().to_string());
    }

    let output = match formatter {
        PathFormatter::Cygwin => Command::new("cygpath").arg(path).output(),
        PathFormatter::Wsl => Command::new("wsl")
            .arg("-e")
            .arg("wslpath")
            .arg(path)
            .output(),
        _ => unreachable!("formatter cannot be another variant"),
    }
    .map_err(|err| anyhow!("Failed to convert Windows path to Unix: {:?}", err))?;

    Ok(std::str::from_utf8(output.stdout.as_slice())?
        .trim()
        .to_string())
}

#[cfg(unix)]
#[allow(clippy::unnecessary_wraps)]
pub fn to_unix_path(path: &Path, formatter: PathFormatter) -> Result<String> {
    Ok(path.to_str().unwrap().to_string())
}

#[cfg(windows)]
pub fn infer_formatter() -> PathFormatter {
    if is_wsl() {
        PathFormatter::Wsl
    } else {
        PathFormatter::Cygwin
    }
}

#[cfg(unix)]
pub fn infer_formatter() -> PathFormatter {
    // already on unix
    PathFormatter::None
}
