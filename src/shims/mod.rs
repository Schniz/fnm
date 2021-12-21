use crate::config::FnmConfig;
use log::info;
use std::io::Write;
use std::path::PathBuf;

#[cfg(not(windows))]
const SHIM_CONTENT: &[u8] = include_bytes!("./unix.sh");

#[cfg(windows)]
const SHIM_CONTENT: &[u8] = include_bytes!("./windows.cmd");

pub type Error = std::io::Error;

/// Creates shims for a path and returns the shim directory
pub fn store_shim(config: &FnmConfig) -> Result<PathBuf, Error> {
    let dir = shims_dir(config)?;
    let executable_path = dir.join(if cfg!(not(windows)) {
        "node"
    } else {
        "node.cmd"
    });

    if executable_path.exists() {
        return Ok(dir);
    }

    info!("Creating a shim at {}", executable_path.display());
    let mut file = std::fs::File::create(&executable_path)?;

    #[cfg(not(windows))]
    {
        use std::os::unix::prelude::PermissionsExt;
        info!("Setting file permissions to 777");
        let mut perm = file.metadata()?.permissions();
        perm.set_mode(0o777);
        file.set_permissions(perm)?;
    };

    file.write_all(SHIM_CONTENT)?;

    Ok(dir)
}

fn shims_dir(config: &FnmConfig) -> Result<PathBuf, std::io::Error> {
    let path = config.base_dir_with_default().join("shims");
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_shim() {
        let base_dir = tempdir().unwrap();
        let config = FnmConfig::default().with_base_dir(Some(base_dir.into_path()));
        let dir = shims_dir(&config).unwrap();
        assert!(dir.exists());
    }
}
