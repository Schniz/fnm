use crate::config::FnmConfig;
use crate::fs::{remove_symlink_dir, shallow_read_symlink, symlink_dir};
use crate::system_version;
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

    let version_dir = version.installation_path(config);
    let alias_dir = aliases_dir.join(common_name);

    remove_symlink_dir(&alias_dir).ok();
    symlink_dir(version_dir, &alias_dir)?;

    Ok(())
}

pub fn list_aliases(config: &FnmConfig) -> std::io::Result<Vec<StoredAlias>> {
    let vec: Vec<_> = std::fs::read_dir(config.aliases_dir())?
        .filter_map(Result::ok)
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
        let shallow_self = shallow_read_symlink(self)?;
        let destination_path = if shallow_self == system_version::path() {
            shallow_self
        } else {
            std::fs::canonicalize(&shallow_self)?
        };
        Ok(StoredAlias {
            alias_path: PathBuf::from(self),
            destination_path,
        })
    }
}

impl StoredAlias {
    pub fn s_ver(&self) -> &str {
        if self.destination_path == system_version::path() {
            system_version::display_name()
        } else {
            self.destination_path
                .parent()
                .unwrap()
                .file_name()
                .expect("must have basename")
                .to_str()
                .unwrap()
        }
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
