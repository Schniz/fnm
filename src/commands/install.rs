use crate::alias::create_alias;
use crate::arch;
use crate::config::FnmConfig;
use crate::downloader::{install_node_dist, Error as DownloaderError};
use crate::lts::LtsType;
use crate::outln;
use crate::remote_node_index;
use crate::user_version::UserVersion;
use crate::version::Version;
use crate::version_files::get_user_version_for_directory;
use colored::Colorize;
use log::debug;
use snafu::{ensure, OptionExt, ResultExt, Snafu};
use structopt::StructOpt;

#[derive(StructOpt, Debug, Default)]
pub struct Install {
    /// A version string. Can be a partial semver or a LTS version name by the format lts/NAME
    pub version: Option<UserVersion>,

    /// Install latest LTS
    #[structopt(long, conflicts_with = "version")]
    pub lts: bool,

    /// Install specified architecture
    #[structopt(long)]
    pub arch: Option<arch::Arch>,
}

impl Install {
    fn version(self) -> Result<Option<UserVersion>, Error> {
        match self {
            Self {
                version: Some(_),
                lts: true,
                arch: _,
            } => Err(Error::TooManyVersionsProvided),
            Self {
                version: v,
                lts: false,
                arch: _,
            } => Ok(v),
            Self {
                version: None,
                lts: true,
                arch: _,
            } => Ok(Some(UserVersion::Full(Version::Lts(LtsType::Latest)))),
        }
    }
}

impl super::command::Command for Install {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let current_dir = std::env::current_dir().unwrap();

        let arch = match &self.arch {
            Some(arch) => arch.clone(),
            None => arch::get_default(),
        };

        let current_version = self
            .version()?
            .or_else(|| get_user_version_for_directory(current_dir))
            .context(CantInferVersion)?;

        let version = match current_version.clone() {
            UserVersion::Full(Version::Semver(actual_version)) => Version::Semver(actual_version),
            UserVersion::Full(v @ Version::Bypassed) | UserVersion::Full(v @ Version::Alias(_)) => {
                ensure!(false, UninstallableVersion { version: v });
                unreachable!();
            }
            UserVersion::Full(Version::Lts(lts_type)) => {
                let available_versions: Vec<_> = remote_node_index::list(&config.node_dist_mirror)
                    .context(CantListRemoteVersions)?;
                let picked_version = lts_type
                    .pick_latest(&available_versions)
                    .with_context(|| CantFindRelevantLts {
                        lts_type: lts_type.clone(),
                    })?
                    .version
                    .clone();
                debug!(
                    "Resolved {} into Node version {}",
                    Version::Lts(lts_type).v_str().cyan(),
                    picked_version.v_str().cyan()
                );
                picked_version
            }
            current_version => {
                let available_versions: Vec<_> = remote_node_index::list(&config.node_dist_mirror)
                    .context(CantListRemoteVersions)?
                    .drain(..)
                    .map(|x| x.version)
                    .collect();

                current_version
                    .to_version(&available_versions, &config)
                    .context(CantFindNodeVersion {
                        requested_version: current_version,
                    })?
                    .clone()
            }
        };
        let version_str = format!("Node {}", &version);
        outln!(config#Info, "Installing {}", version_str.cyan());
        match install_node_dist(
            &version,
            &config.node_dist_mirror,
            config.installations_dir(),
            &arch,
        ) {
            Err(err @ DownloaderError::VersionAlreadyInstalled { .. }) => {
                outln!(config#Error, "{} {}", "warning:".bold().yellow(), err);
            }
            other_err => other_err.context(DownloadError)?,
        };

        if let UserVersion::Full(Version::Lts(lts_type)) = current_version {
            let alias_name = Version::Lts(lts_type).v_str();
            debug!(
                "Tagging {} as alias for {}",
                alias_name.cyan(),
                version.v_str().cyan()
            );
            create_alias(&config, &alias_name, &version).context(IoError)?;
        }

        if !config.default_version_dir().exists() {
            debug!("Tagging {} as the default version", version.v_str().cyan());
            create_alias(&config, "default", &version).context(IoError)?;
        }

        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Can't download the requested version: {}", source))]
    DownloadError {
        source: DownloaderError,
    },
    IoError {
        source: std::io::Error,
    },
    #[snafu(display(
        "Can't find version in dotfiles. Please provide a version manually to the command."
    ))]
    CantInferVersion,
    #[snafu(display("Having a hard time listing the remote versions: {}", source))]
    CantListRemoteVersions {
        source: reqwest::Error,
    },
    #[snafu(display(
        "Can't find a Node version that matches {} in remote",
        requested_version
    ))]
    CantFindNodeVersion {
        requested_version: UserVersion,
    },
    #[snafu(display("Can't find relevant LTS named {}", lts_type))]
    CantFindRelevantLts {
        lts_type: crate::lts::LtsType,
    },
    #[snafu(display("The requested version is not installable: {}", version.v_str()))]
    UninstallableVersion {
        version: Version,
    },
    #[snafu(display("Too many versions provided. Please don't use --lts with a version string."))]
    TooManyVersionsProvided,
}

#[cfg(test)]
mod tests {
    use super::super::command::Command;
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn test_set_default_on_new_installation() {
        let base_dir = tempfile::tempdir().unwrap();
        let config = FnmConfig::default().with_base_dir(Some(base_dir.path().to_path_buf()));
        assert!(!config.default_version_dir().exists());

        Install {
            version: UserVersion::from_str("12.0.0").ok(),
            lts: false,
            arch: Some(arch::get_default()),
        }
        .apply(&config)
        .expect("Can't install");

        assert!(config.default_version_dir().exists());
        assert_eq!(
            config.default_version_dir().canonicalize().ok(),
            config
                .installations_dir()
                .join("v12.0.0")
                .join("installation")
                .canonicalize()
                .ok()
        );
    }
}
