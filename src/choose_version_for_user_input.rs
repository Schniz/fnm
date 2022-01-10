use crate::config::FnmConfig;
use crate::fs;
use crate::installed_versions;
use crate::system_version;
use crate::user_version::UserVersion;
use crate::version::Version;
use colored::Colorize;
use log::info;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug)]
pub struct ApplicableVersion {
    path: PathBuf,
    version: Version,
}

impl ApplicableVersion {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn version(&self) -> &Version {
        &self.version
    }
}

pub fn choose_version_for_user_input<'a>(
    requested_version: &'a UserVersion,
    config: &'a FnmConfig,
) -> Result<Option<ApplicableVersion>, Error> {
    let all_versions = installed_versions::list(config.installations_dir())
        .map_err(|source| Error::VersionListing { source })?;

    let result = if let UserVersion::Full(Version::Bypassed) = requested_version {
        info!(
            "Bypassing fnm: using {} node",
            system_version::display_name().cyan()
        );
        Some(ApplicableVersion {
            path: system_version::path(),
            version: Version::Bypassed,
        })
    } else if let Some(alias_name) = requested_version.alias_name() {
        let alias_path = config.aliases_dir().join(&alias_name);
        let system_path = system_version::path();
        if matches!(fs::shallow_read_symlink(&alias_path), Ok(shallow_path) if shallow_path == system_path)
        {
            info!(
                "Bypassing fnm: using {} node",
                system_version::display_name().cyan()
            );
            Some(ApplicableVersion {
                path: alias_path,
                version: Version::Bypassed,
            })
        } else if alias_path.exists() {
            info!("Using Node for alias {}", alias_name.cyan());
            Some(ApplicableVersion {
                path: alias_path,
                version: Version::Alias(alias_name),
            })
        } else {
            return Err(Error::CantFindVersion {
                requested_version: requested_version.clone(),
            });
        }
    } else {
        let current_version = requested_version.to_version(&all_versions, config);
        current_version.map(|version| {
            info!("Using Node {}", version.to_string().cyan());
            let path = config
                .installations_dir()
                .join(version.to_string())
                .join("installation");

            ApplicableVersion {
                path,
                version: version.clone(),
            }
        })
    };

    Ok(result)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't find requested version: {}", requested_version)]
    CantFindVersion { requested_version: UserVersion },
    #[error("Can't list local installed versions: {}", source)]
    VersionListing { source: installed_versions::Error },
}
