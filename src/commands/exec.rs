use super::command::Command as Cmd;
use crate::choose_version_for_user_input::choose_version_for_user_input;
use crate::choose_version_for_user_input::Error as UserInputError;
use crate::config::FnmConfig;
use crate::user_version::UserVersion;
use crate::version_files::get_user_version_from_file;
use snafu::{OptionExt, ResultExt, Snafu};
use std::process::{Command, Stdio};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Exec {
    binary: String,
    arguments: Vec<String>,
    #[structopt(long = "using")]
    version: Option<UserVersion>,
}

impl Cmd for Exec {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let version = self
            .version
            .or_else(|| {
                let current_dir = std::env::current_dir().unwrap();
                get_user_version_from_file(current_dir)
            })
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

        let exit_status = Command::new(&self.binary)
            .args(self.arguments)
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
}
