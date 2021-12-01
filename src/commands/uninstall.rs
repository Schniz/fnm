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
use snafu::{ensure, OptionExt, ResultExt, Snafu};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Uninstall {
    version: Option<UserVersion>,
}

impl Command for Uninstall {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let all_versions =
            installed_versions::list(config.installations_dir()).context(VersionListingError)?;
        let requested_version = self
            .version
            .or_else(|| {
                let current_dir = std::env::current_dir().unwrap();
                get_user_version_for_directory(current_dir)
            })
            .context(CantInferVersion)?;

        ensure!(
            !matches!(requested_version, UserVersion::Full(Version::Bypassed)),
            CantUninstallSystemVersion
        );

        let available_versions: Vec<&Version> = all_versions
            .iter()
            .filter(|v| requested_version.matches(v, config))
            .collect();

        ensure!(
            available_versions.len() < 2,
            PleaseBeMoreSpecificToDelete {
                matched_versions: available_versions
                    .iter()
                    .map(|x| x.v_str())
                    .collect::<Vec<_>>()
            }
        );

        let version = requested_version
            .to_version(&all_versions, config)
            .context(CantFindVersion)?;

        let matching_aliases = version.find_aliases(config).context(IoError)?;
        let root_path = version
            .root_path(config)
            .with_context(|| RootPathNotFound {
                version: version.clone(),
            })?;

        debug!("Removing Node version from {:?}", root_path);
        std::fs::remove_dir_all(root_path).context(CantDeleteNodeVersion)?;
        outln!(config#Info, "Node version {} was removed successfully", version.v_str().cyan());

        for alias in matching_aliases {
            debug!("Removing alias from {:?}", alias.path());
            remove_symlink_dir(alias.path()).context(CantDeleteSymlink)?;
            outln!(config#Info, "Alias {} was removed successfully", alias.name().cyan());
        }

        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Can't get locally installed versions: {}", source))]
    VersionListingError { source: installed_versions::Error },
    #[snafu(display(
        "Can't find version in dotfiles. Please provide a version manually to the command."
    ))]
    CantInferVersion,
    #[snafu(display("Can't uninstall system version"))]
    CantUninstallSystemVersion,
    #[snafu(display("Too many versions had matched, please be more specific.\nFound {} matching versions, expected 1:\n{}", matched_versions.len(), matched_versions.iter().map(|v| format!("* {}", v)).collect::<Vec<_>>().join("\n")))]
    PleaseBeMoreSpecificToDelete { matched_versions: Vec<String> },
    #[snafu(display("Can't find a matching version"))]
    CantFindVersion,
    #[snafu(display("Root path not found for version {}", version))]
    RootPathNotFound { version: Version },
    #[snafu(display("io error: {}", source))]
    IoError { source: std::io::Error },
    #[snafu(display("Can't delete Node.js version: {}", source))]
    CantDeleteNodeVersion { source: std::io::Error },
    #[snafu(display("Can't delete symlink: {}", source))]
    CantDeleteSymlink { source: std::io::Error },
}
