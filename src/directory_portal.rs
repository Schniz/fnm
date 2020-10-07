use log::*;
use std::path::Path;
use tempfile::{tempdir, TempDir};

pub struct DirectoryPortal<P: AsRef<Path>> {
    temp_dir: TempDir,
    target: P,
}

impl<P: AsRef<Path>> DirectoryPortal<P> {
    #[must_use]
    pub fn new(target: P) -> Self {
        let temp_dir = tempdir().expect("Can't generate a temp directory");
        debug!("Created a temp directory in {:?}", temp_dir.path());
        Self { target, temp_dir }
    }

    pub fn teleport(self) -> std::io::Result<P> {
        debug!(
            "Moving directory {:?} into {:?}",
            self.temp_dir.path(),
            self.target.as_ref()
        );
        std::fs::rename(&self.temp_dir, &self.target)?;
        Ok(self.target)
    }
}

impl<P: AsRef<Path>> std::ops::Deref for DirectoryPortal<P> {
    type Target = Path;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<P: AsRef<Path>> AsRef<Path> for DirectoryPortal<P> {
    fn as_ref(&self) -> &Path {
        self.temp_dir.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_portal() {
        let tempdir = tempdir().expect("Can't generate a temp directory");
        let portal = DirectoryPortal::new(tempdir.path().join("subdir"));
        let new_file_path = portal.to_path_buf().join("README.md");
        std::fs::write(&new_file_path, "Hello world!").expect("Can't write file");
        let target = portal.teleport().expect("Can't close directory portal");

        let file_exists: Vec<_> = target
            .read_dir()
            .expect("Can't read dir")
            .map(|x| x.unwrap().file_name().into_string().unwrap())
            .collect();

        assert_eq!(file_exists, vec!["README.md"]);
    }
}
