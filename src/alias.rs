use crate::config::FnmConfig;
use crate::fs::{remove_symlink_dir, symlink_dir};
use crate::version::Version;
use std::convert::TryInto;
use std::path::PathBuf;

pub fn create_alias(
    config: &FnmConfig,
    common_name: &str,
    version: &Version,
) -> std::io::Result<()> {
    let aliases_dir = config.aliases_dir();
    std::fs::create_dir_all(&aliases_dir)?;

    let version_dir = version
        .installation_path(config)
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound));
    let alias_dir = aliases_dir.join(common_name);

    if alias_dir.exists() {
        remove_symlink_dir(&alias_dir)?;
    }

    if *version != Version::Bypassed {
        let version_dir_res = version_dir?;
        // Symlink only if version should not bypass system node
        symlink_dir(&version_dir_res, &alias_dir)?;
    }

    Ok(())
}

pub fn list_aliases(config: &FnmConfig) -> std::io::Result<Vec<StoredAlias>> {
    let vec: Vec<_> = std::fs::read_dir(&config.aliases_dir())?
        .filter_map(|item| item.ok())
        .filter_map(|x| TryInto::<StoredAlias>::try_into(x.path().as_path()).ok())
        .collect();
    Ok(vec)
}

#[derive(Debug)]
pub struct StoredAlias {
    alias_path: PathBuf,
    destination_path: PathBuf,
}

impl std::convert::TryInto<StoredAlias> for &std::path::Path {
    type Error = std::io::Error;

    fn try_into(self) -> Result<StoredAlias, Self::Error> {
        let destination_path = std::fs::canonicalize(&self)?;
        Ok(StoredAlias {
            alias_path: PathBuf::from(self),
            destination_path,
        })
    }
}

impl StoredAlias {
    pub fn s_ver(&self) -> &str {
        self.destination_path
            .parent()
            .unwrap()
            .file_name()
            .expect("must have basename")
            .to_str()
            .unwrap()
    }

    pub fn name(&self) -> &str {
        self.alias_path
            .file_name()
            .expect("must have basename")
            .to_str()
            .unwrap()
    }

    pub fn path(&self) -> &std::path::Path {
        &self.alias_path
    }
}
