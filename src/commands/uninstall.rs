use super::command::Command;
use crate::config::FnmConfig;
use crate::fs::remove_symlink_dir;
use crate::installed_versions;
use crate::outln;
use crate::user_version::UserVersion;
use crate::version::Version;
use crate::version_files::get_user_version_for_directory;
use colored::Colorize;
use log::debug;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct Uninstall {
    version: Option<UserVersion>,
}

impl Command for Uninstall {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let all_versions = installed_versions::list(config.installations_dir())
            .map_err(|source| Error::VersionListingError { source })?;
        let requested_version = self
            .version
            .or_else(|| {
                let current_dir = std::env::current_dir().unwrap();
                get_user_version_for_directory(current_dir, config)
            })
            .ok_or(Error::CantInferVersion)?;

        if matches!(requested_version, UserVersion::Full(Version::Bypassed)) {
            return Err(Error::CantUninstallSystemVersion);
        }

        let available_versions: Vec<&Version> = all_versions
            .iter()
            .filter(|v| requested_version.matches(v, config))
            .collect();

        if available_versions.len() >= 2 {
            return Err(Error::PleaseBeMoreSpecificToDelete {
                matched_versions: available_versions
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            });
        }

        let version = requested_version
            .to_version(&all_versions, config)
            .ok_or(Error::CantFindVersion)?;

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

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't get locally installed versions: {}", source)]
    VersionListingError { source: installed_versions::Error },
    #[error("Can't find version in dotfiles. Please provide a version manually to the command.")]
    CantInferVersion,
    #[error("Can't uninstall system version")]
    CantUninstallSystemVersion,
    #[error("Too many versions had matched, please be more specific.\nFound {} matching versions, expected 1:\n{}", matched_versions.len(), matched_versions.iter().map(|v| format!("* {v}")).collect::<Vec<_>>().join("\n"))]
    PleaseBeMoreSpecificToDelete { matched_versions: Vec<String> },
    #[error("Can't find a matching version")]
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
