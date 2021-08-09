use crate::alias::create_alias;
use crate::arch::get_safe_arch;
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
    /// Install global packages from an existing version
    #[structopt(long)]
    pub reinstall_packages_from: Option<UserVersion>,

    /// A version string. Can be a partial semver or a LTS version name by the format lts/NAME
    pub version: Option<UserVersion>,

    /// Install latest LTS
    #[structopt(long, conflicts_with = "version")]
    pub lts: bool,
}

impl Install {
    fn version(&self) -> Result<Option<UserVersion>, Error> {
        match &self {
            Self {
                reinstall_packages_from: _,
                version: Some(_),
                lts: true,
            } => Err(Error::TooManyVersionsProvided),
            Self {
                reinstall_packages_from: _,
                version: v,
                lts: false,
            } => Ok(v.clone()),
            Self {
                reinstall_packages_from: _,
                version: None,
                lts: true,
            } => Ok(Some(UserVersion::Full(Version::Lts(LtsType::Latest)))),
        }
    }
}

impl super::command::Command for Install {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let current_dir = std::env::current_dir().unwrap();

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

        // Automatically swap Apple Silicon to x64 arch for appropriate versions.
        let safe_arch = get_safe_arch(&config.arch, &version);

        let version_str = format!("Node {}", &version);
        outln!(config#Info, "Installing {} ({})", version_str.cyan(), safe_arch.to_string());

        /*
        Prepare reinstalling packages.
         */
        let install_packages_string_option = (match self.reinstall_packages_from {
            Some(user_version) => {
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!(
                        concat!(
                        "fnm use \"{}\" >/dev/null ",
                        "&& npm list -g --depth=0 2>/dev/null ",
                        "| command sed 1,1d ",
                        "| command sed ",
                        "-e '/ -> / d' ",
                        "-e '/\\(empty\\)/ d' ",
                        "-e 's/^.* \\(.*@[^ ]*\\).*/\\1/' ",
                        "-e '/^npm@[^ ]*.*$/ d'",
                        ),
                        user_version,
                    ))
                    .output().expect("Failed to get package list.");
                if output.status.success() {
                    Ok(Some(std::string::String::from_utf8(output.stdout).unwrap()))
                } else {
                    Err(crate::commands::install::Error::CommandExecutionFailed {
                        stderr: std::string::String::from_utf8(output.stderr).unwrap(),
                    })
                }
            },
            None => Ok(None),
        })?;

        match install_node_dist(
            &version,
            &config.node_dist_mirror,
            config.installations_dir(),
            safe_arch,
        ) {
            Err(err @ DownloaderError::VersionAlreadyInstalled { .. }) => {
                outln!(config#Error, "{} {}", "warning:".bold().yellow(), err);
            }
            other_err => other_err.context(DownloadError)?,
        };

        if let UserVersion::Full(Version::Lts(lts_type)) = current_version.clone() {
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

        /*
        Reinstall packages.
         */
        (match install_packages_string_option {
            Some(packages_string) => {
                outln!(config#Info, "Installing packages:\n{}", packages_string);
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!(
                        concat!(
                        "fnm use \"{new_version}\" >/dev/null ",
                        "&& echo \"{packages_string}\" ",
                        "| command xargs npm install -g --quiet",
                        ),
                        new_version = current_version,
                        packages_string = packages_string,
                    ))
                    .output()
                    .expect("Failed to install packages.");
                if output.status.success() {
                    outln!(config#Info,"{}", std::str::from_utf8(&output.stdout).unwrap());
                    Ok(())
                } else {
                    Err(crate::commands::install::Error::CommandExecutionFailed {
                        stderr: std::string::String::from_utf8(output.stderr).unwrap(),
                    })
                }
            }
            None => Ok(())
        })?;

        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Can't download the requested binary: {}", source))]
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
    #[snafu(display("Execute command failed:\n{}", stderr))]
    CommandExecutionFailed {
        stderr: String,
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
            reinstall_packages_from: None,
            version: UserVersion::from_str("12.0.0").ok(),
            lts: false,
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
