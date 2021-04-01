pub trait PathExt {
    fn ensure_exists_silently(self) -> Self;
}

impl<T: AsRef<std::path::Path>> PathExt for T {
    /// Ensures a path is existing by creating it recursively
    /// if it is missing. No error is emitted if the creation has failed.
    fn ensure_exists_silently(self) -> Self {
        std::fs::create_dir_all(self.as_ref()).ok();
        self
    }
}
