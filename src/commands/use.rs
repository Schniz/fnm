use super::command::Command;
use super::install::Install;
use crate::fs;
use crate::installed_versions;
use crate::outln;
use crate::system_version;
use crate::user_version::UserVersion;
use crate::version::Version;
use crate::{config::FnmConfig, user_version_reader::UserVersionReader};
use colored::Colorize;
use snafu::{ensure, OptionExt, ResultExt, Snafu};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Use {
    version: Option<UserVersionReader>,
    /// Install the version if it isn't installed yet
    #[structopt(long)]
    install_if_missing: bool,
}

impl Command for Use {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let multishell_path = config.multishell_path().context(FnmEnvWasNotSourced)?;
        warn_if_multishell_path_not_in_path_env_var(&multishell_path, &config);

        let all_versions =
            installed_versions::list(config.installations_dir()).context(VersionListingError)?;
        let requested_version = self
            .version
            .unwrap_or_else(|| {
                let current_dir = std::env::current_dir().unwrap();
                UserVersionReader::Path(current_dir)
            })
            .to_user_version()
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
            let current_version = requested_version.to_version(&all_versions, &config);
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
                        version: Some(UserVersionReader::Direct(requested_version)),
                        install_if_missing: self.install_if_missing,
                    }
                    .apply(config)?;

                    return Ok(());
                }
            }
        };

        replace_symlink(&version_path, &multishell_path).context(SymlinkingCreationIssue)?;

        Ok(())
    }
}

/// Tries to delete `from`, and then tries to symlink `from` to `to` anyway.
/// If the symlinking fails, it will return the errors in the following order:
/// * The deletion error (if exists)
/// * The creation error
///
/// This way, we can create a symlink if it is missing.
fn replace_symlink(from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
    let symlink_deletion_result = fs::remove_symlink_dir(&to);
    match fs::symlink_dir(&from, &to) {
        ok @ Ok(_) => ok,
        err @ Err(_) => symlink_deletion_result.and(err),
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

fn warn_if_multishell_path_not_in_path_env_var(
    multishell_path: &std::path::Path,
    config: &FnmConfig,
) {
    let bin_path = if cfg!(unix) {
        multishell_path.join("bin")
    } else {
        multishell_path.to_path_buf()
    };

    for path in std::env::split_paths(&std::env::var("PATH").unwrap_or(String::new())) {
        if bin_path == path {
            return;
        }
    }

    outln!(
        config#Error,
        "{} {}\n{}\n{}",
        "warning:".yellow().bold(),
        "The current Node.js path is not on your PATH environment variable.".yellow(),
        "Have you set up your shell profile to evaluate `fnm env`?".yellow(),
        "Check out our documentation for more information: https://fnm.vercel.app".yellow()
    );
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Can't create the symlink: {}", source))]
    SymlinkingCreationIssue { source: std::io::Error },
    #[snafu(display("{}", source))]
    InstallError { source: <Install as Command>::Error },
    #[snafu(display("Can't get locally installed versions: {}", source))]
    VersionListingError { source: installed_versions::Error },
    #[snafu(display("Requested version {} is not currently installed", version))]
    CantFindVersion { version: UserVersion },
    #[snafu(display(
        "Can't find version in dotfiles. Please provide a version manually to the command."
    ))]
    CantInferVersion,
    #[snafu(display(
        "{}\n{}\n{}",
        "We can't find the necessary environment variables to replace the Node version.",
        "Have you set up your shell profile to evaluate `fnm env`?",
        "Check out our documentation for more information: https://fnm.vercel.app"
    ))]
    FnmEnvWasNotSourced,
}
