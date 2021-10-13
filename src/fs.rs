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

#[cfg(unix)]
pub fn shallow_read_symlink<P: AsRef<Path>>(path: P) -> std::io::Result<std::path::PathBuf> {
    std::fs::read_link(path)
}

#[cfg(windows)]
pub fn shallow_read_symlink<P: AsRef<Path>>(path: P) -> std::io::Result<std::path::PathBuf> {
    junction::get_target(path)
}
