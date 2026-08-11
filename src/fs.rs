use std::path::Path;

#[cfg(unix)]
pub fn symlink_dir<P: AsRef<Path>, U: AsRef<Path>>(from: P, to: U) -> std::io::Result<()> {
    std::os::unix::fs::symlink(from, to)?;
    Ok(())
}

#[cfg(windows)]
pub fn symlink_dir<P: AsRef<Path>, U: AsRef<Path>>(from: P, to: U) -> std::io::Result<()> {
    junction::create(from, to)?;
    Ok(())
}

#[cfg(windows)]
pub fn remove_symlink_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    std::fs::remove_dir(path)?;
    Ok(())
}

#[cfg(unix)]
pub fn remove_symlink_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    std::fs::remove_file(path)?;
    Ok(())
}

pub fn shallow_read_symlink<P: AsRef<Path>>(path: P) -> std::io::Result<std::path::PathBuf> {
    std::fs::read_link(path)
}

/// Creates a symlink into a temporary directory,
/// then renames the temporary directory to the target.
///
/// This is required as symlinking doesn't override existing
/// symlinks.
/// The naive solution is to delete, and then symlink, but that can create
/// a race condition in the filesystem.
pub fn two_phase_symlink(
    source: &std::path::Path,
    target: &std::path::Path,
) -> Result<(), TwoPhaseSymlinkError> {
    let temp = tempfile::tempdir().map_err(TwoPhaseSymlinkError::CreatingTempSymlink)?;
    let temp_path = temp.path().join("temp");
    symlink_dir(source, &temp_path).map_err(TwoPhaseSymlinkError::CreatingTempSymlink)?;
    std::fs::rename(&temp_path, target).map_err(TwoPhaseSymlinkError::Renaming)
}

#[derive(Debug, thiserror::Error)]
pub enum TwoPhaseSymlinkError {
    #[error("Can't create temporary symlink for two phase symlink: {0}")]
    CreatingTempSymlink(#[source] std::io::Error),
    #[error("Can't create a symlink in the target directory: {0}")]
    Renaming(#[source] std::io::Error),
}
