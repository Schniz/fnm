use super::command::Command as Cmd;
use crate::choose_version_for_user_input::{
    choose_version_for_user_input, Error as UserInputError,
};
use crate::config::FnmConfig;
use crate::outln;
use crate::user_version::UserVersion;
use crate::user_version_reader::UserVersionReader;
use colored::Colorize;
use snafu::{OptionExt, ResultExt, Snafu};
use std::process::{Command, Stdio};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::TrailingVarArg)]
pub struct Exec {
    /// Either an explicit version, or a filename with the version written in it
    #[structopt(long = "using")]
    version: Option<UserVersionReader>,
    /// Deprecated. This is the default now.
    #[structopt(long = "using-file", hidden = true)]
    using_file: bool,
    /// The command to run
    arguments: Vec<String>,
}

impl Cmd for Exec {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        if self.using_file {
            outln!(config#Error, "{} {} is deprecated. This is now the default.", "warning:".yellow().bold(), "--using-file".italic());
        }

        let (binary, arguments) = self.arguments.split_first().context(NoBinaryProvided)?;

        let version = self
            .version
            .unwrap_or_else(|| {
                let current_dir = std::env::current_dir().unwrap();
                UserVersionReader::Path(current_dir)
            })
            .to_user_version()
            .context(CantInferVersion)?;

        let applicable_version = choose_version_for_user_input(&version, &config)
            .context(ApplicableVersionError)?
            .context(VersionNotFound { version })?;

        #[cfg(windows)]
        let bin_path = applicable_version.path().to_path_buf();

        #[cfg(unix)]
        let bin_path = applicable_version.path().join("bin");

        let path_env = {
            let paths_env = std::env::var_os("PATH").context(CantReadPathVariable)?;
            let mut paths: Vec<_> = std::env::split_paths(&paths_env).collect();
            paths.insert(0, bin_path);
            std::env::join_paths(paths).context(CantAddPathToEnvironment)?
        };

        let exit_status = Command::new(&binary)
            .args(arguments)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .env("PATH", path_env)
            .spawn()
            .expect("Can't spawn program")
            .wait()
            .expect("Failed to grab exit code");

        std::process::exit(exit_status.code().unwrap());
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Can't read path environment variable"))]
    CantReadPathVariable,
    #[snafu(display("Can't add path to environment variable: {}", source))]
    CantAddPathToEnvironment {
        source: std::env::JoinPathsError,
    },
    #[snafu(display(
        "Can't find version in dotfiles. Please provide a version manually to the command."
    ))]
    CantInferVersion,
    #[snafu(display("Requested version {} is not currently installed", version))]
    VersionNotFound {
        version: UserVersion,
    },
    ApplicableVersionError {
        source: UserInputError,
    },
    #[snafu(display("command not provided. Please provide a command to run as an argument, like {} or {}.\n{} {}", "node".italic(), "bash".italic(), "example:".yellow().bold(), "fnm exec --using=12 node --version".italic().yellow()))]
    NoBinaryProvided,
}
