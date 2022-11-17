use std::path::Path;

use thiserror::Error;

use crate::config::FnmConfig;
use crate::version::Version;
use crate::{default_version, installed_versions, system_version, version_files};

pub fn current_version(config: &FnmConfig) -> Result<Option<Version>, Error> {
    match config.multishell_path() {
        None => infer_current_version(config),
        Some(multishell_path) => current_version_from_multishell(config, multishell_path),
    }
}

/// infer the current version from the system state,
/// by looking at the installed versions, the current working
/// directory and the default version
fn infer_current_version(config: &FnmConfig) -> Result<Option<Version>, Error> {
    let current_directory =
        std::env::current_dir().map_err(|err| Error::CantAccessCurrentDirectory { source: err })?;

    if let Some(version) = version_files::get_user_version_for_directory(current_directory, config)
    {
        let all_versions = installed_versions::list(config.installations_dir())
            .map_err(|source| Error::VersionListingError { source })?;
        return Ok(version.to_version(&all_versions, config).cloned());
    }

    if let Some(version) = default_version::find_default_version(config) {
        return Ok(Some(version));
    }

    return Ok(None);
}

fn current_version_from_multishell(
    _config: &FnmConfig,
    multishell_path: &Path,
) -> Result<Option<Version>, Error> {
    if multishell_path.read_link().ok() == Some(system_version::path()) {
        return Ok(Some(Version::Bypassed));
    }

    if let Ok(resolved_path) = std::fs::canonicalize(multishell_path) {
        let installation_path = resolved_path
            .parent()
            .expect("multishell path can't be in the root");
        let file_name = installation_path
            .file_name()
            .expect("Can't get filename")
            .to_str()
            .expect("Invalid OS string");
        let version = Version::parse(file_name).map_err(|source| Error::VersionError {
            source,
            version: file_name.to_string(),
        })?;
        Ok(Some(version))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't read the version as a valid semver")]
    VersionError {
        source: semver::Error,
        version: String,
    },
    #[error("Can't access current directory")]
    CantAccessCurrentDirectory { source: std::io::Error },
    #[error("Can't list versions")]
    VersionListingError { source: installed_versions::Error },
}
