use log::warn;
use std::fs::Permissions;

pub trait PathExt {
    fn exists(&self) -> bool;
    fn ensure_exists_silently(self) -> Self;
    fn ensure_exists_silently_with_permissions<F>(self, permissions: F) -> Self
    where
        F: FnOnce(&mut Permissions);
}

impl<T: AsRef<std::path::Path>> PathExt for T {
    fn exists(&self) -> bool {
        std::path::Path::exists(self.as_ref())
    }

    /// Ensures a path is existing by creating it recursively
    /// if it is missing. No error is emitted if the creation has failed.
    fn ensure_exists_silently(self) -> Self {
        if let Err(err) = std::fs::create_dir_all(self.as_ref()) {
            warn!("Failed to create directory {:?}: {}", self.as_ref(), err);
        }
        self
    }

    fn ensure_exists_silently_with_permissions<F>(self, modify_permissions: F) -> Self
    where
        F: FnOnce(&mut Permissions),
    {
        if self.exists() {
            return self;
        }

        if let Err(err) = std::fs::create_dir_all(self.as_ref()) {
            warn!("Failed to create directory {:?}: {err}", self.as_ref());
        }

        let modified = self.as_ref().metadata().and_then(|x| {
            let mut permissions = x.permissions();
            modify_permissions(&mut permissions);
            std::fs::set_permissions(self.as_ref(), permissions)
        });

        if let Err(err) = modified {
            warn!("Failed to set permissions {:?}: {err}", self.as_ref());
        }

        self
    }
}
