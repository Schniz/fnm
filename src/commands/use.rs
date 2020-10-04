use super::command::Command;
use super::install::Install;
use crate::config::FnmConfig;
use crate::fs;
use crate::installed_versions;
use crate::outln;
use crate::system_version;
use crate::user_version::UserVersion;
use crate::version::Version;
use crate::version_files::get_user_version_from_file;
use colored::Colorize;
use snafu::{ensure, OptionExt, ResultExt, Snafu};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Use {
    version: Option<UserVersion>,
    /// Install the version if it isn't installed yet
    #[structopt(long)]
    install_if_missing: bool,
}

impl Command for Use {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let all_versions =
            installed_versions::list(config.installations_dir()).context(VersionListingError)?;
        let requested_version = self
            .version
            .or_else(|| {
                let current_dir = std::env::current_dir().unwrap();
                get_user_version_from_file(current_dir)
            })
            .context(CantInferVersion)?;

        let version_path = if let UserVersion::Full(Version::Bypassed) = requested_version {
            outln!(config#Info, "Bypassing fnm: using {} node", "system".cyan());
            system_version::path()
        } else if let Some(alias_name) = requested_version.alias_name() {
            let alias_path = config.aliases_dir().join(&alias_name);
            ensure!(
                alias_path.exists(),
                CantFindVersion {
                    version: requested_version
                }
            );
            outln!(config#Info, "Using Node for alias {}", alias_name.cyan());
            alias_path
        } else {
            let current_version = requested_version.to_version(&all_versions);
            match current_version {
                Some(version) => {
                    outln!(config#Info, "Using Node {}", version.to_string().cyan());
                    config
                        .installations_dir()
                        .join(version.to_string())
                        .join("installation")
                }
                None => {
                    ensure!(
                        self.install_if_missing || should_install_interactively(&requested_version),
                        CantFindVersion {
                            version: requested_version
                        }
                    );

                    Install {
                        version: Some(requested_version.clone()),
                        ..Default::default()
                    }
                    .apply(config)
                    .context(InstallError)?;

                    Self {
                        version: Some(requested_version),
                        install_if_missing: self.install_if_missing,
                    }
                    .apply(config)?;

                    return Ok(());
                }
            }
        };

        let multishell_path = config
            .multishell_path()
            .expect("fnm isn't set up. Have you tried running `fnm env`?");

        fs::remove_symlink_dir(&multishell_path).context(SymlinkingDeletionIssue)?;
        fs::symlink_dir(version_path, &multishell_path).context(SymlinkingCreationIssue)?;

        Ok(())
    }
}

fn should_install_interactively(requested_version: &UserVersion) -> bool {
    if !(atty::is(atty::Stream::Stdout) && atty::is(atty::Stream::Stdin)) {
        return false;
    }

    use std::io::Write;
    let error_message = format!(
        "Can't find version that matches {}.",
        requested_version.to_string().italic()
    );
    eprintln!("{}", error_message.red());
    let do_you_want = format!("Do you want to install it? {} [y/n]:", "answer".bold());
    eprint!("{} ", do_you_want.yellow());
    std::io::stdout().flush().unwrap();
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("Can't read user input");

    s.trim().to_lowercase() == "y"
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Can't create the symlink: {}", source))]
    SymlinkingCreationIssue { source: std::io::Error },
    #[snafu(display("Can't delete the symlink: {}", source))]
    SymlinkingDeletionIssue { source: std::io::Error },
    #[snafu(display("{}", source))]
    InstallError { source: <Install as Command>::Error },
    #[snafu(display("Can't get locally installed versions: {}", source))]
    VersionListingError {
        source: crate::installed_versions::Error,
    },
    #[snafu(display("Requested version {} is not currently installed", version))]
    CantFindVersion { version: UserVersion },
    #[snafu(display(
        "Can't find version in dotfiles. Please provide a version manually to the command."
    ))]
    CantInferVersion,
}
