use semver::VersionReq;
use super::command::Command;
use crate::config::FnmConfig;
use crate::fs::remove_symlink_dir;
use crate::installed_versions;
use crate::outln;
use crate::version::Version;
use crate::version_files::get_user_version_for_directory;
use colored::Colorize;
use log::debug;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct Uninstall {
    version: Option<String>,
}

impl Command for Uninstall {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let all_versions = installed_versions::list(config.installations_dir())
            .map_err(|source| Error::VersionListingError { source })?;

        let requested_version_str = self.version.or_else(|| {
            std::env::current_dir()
                .ok()
                .and_then(|dir| get_user_version_for_directory(dir, config).map(|v| v.to_string()))
        })
        .ok_or(Error::CantInferVersion)?;

        let requested_version = VersionReq::parse(&requested_version_str.replace("v", ""))
            .map_err(|_| Error::InvalidVersionFormat { version: requested_version_str.clone() })?;

        let matching_versions: Vec<&Version> = all_versions.iter().filter_map(|v| {
            let ver = v.v_str().trim_start_matches('v').to_string();
            semver::Version::parse(&ver)
                .ok()
                .filter(|parsed_version| requested_version.matches(parsed_version))
                .map(|_| v)
        }).collect();

        if matching_versions.is_empty() {
            return Err(Error::CantFindVersion);
        }

        for version in matching_versions {
            let matching_aliases = version.find_aliases(config)?;
            let root_path = version
                .root_path(config)
                .ok_or_else(|| Error::RootPathNotFound {
                version: version.clone(),
            })?;

            debug!("Removing Node version from {:?}", root_path);
            std::fs::remove_dir_all(root_path)
                .map_err(|source| Error::CantDeleteNodeVersion { source })?;
            outln!(
                config,
                Info,
                "Node version {} was removed successfully",
                version.v_str().cyan()
            );

            for alias in matching_aliases {
                debug!("Removing alias from {:?}", alias.path());
                remove_symlink_dir(alias.path())
                    .map_err(|source| Error::CantDeleteSymlink { source })?;
                outln!(
                    config,
                    Info,
                    "Alias {} was removed successfully",
                    alias.name().cyan()
                );
            }
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't get locally installed versions: {}", source)]
    VersionListingError { source: installed_versions::Error },
    #[error("Can't find version in dotfiles. Please provide a version manually to the command.")]
    CantInferVersion,
    #[error("Invalid version format: {}", version)]
    InvalidVersionFormat { version: String },
    #[error("Can't find any matching versions")]
    CantFindVersion,
    #[error("Root path not found for version {}", version)]
    RootPathNotFound { version: Version },
    #[error("io error: {}", source)]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error("Can't delete Node.js version: {}", source)]
    CantDeleteNodeVersion { source: std::io::Error },
    #[error("Can't delete symlink: {}", source)]
    CantDeleteSymlink { source: std::io::Error },
}
