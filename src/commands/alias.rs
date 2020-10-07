use super::command::Command;
use crate::alias::create_alias;
use crate::choose_version_for_user_input::{
    choose_version_for_user_input, Error as ApplicableVersionError,
};
use crate::config::FnmConfig;
use crate::installed_versions;
use crate::user_version::UserVersion;
use snafu::{OptionExt, ResultExt, Snafu};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Alias {
    pub(crate) to_version: UserVersion,
    pub(crate) name: String,
}

impl Command for Alias {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let applicable_version = choose_version_for_user_input(&self.to_version, config)
            .context(CantUnderstandVersion)?
            .context(VersionNotFound {
                version: self.to_version,
            })?;

        create_alias(&config, &self.name, applicable_version.version())
            .context(CantCreateSymlink)?;

        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Can't create symlink for alias: {}", source))]
    CantCreateSymlink { source: std::io::Error },
    #[snafu(display("Can't list local installed versions: {}", source))]
    VersionListingError { source: installed_versions::Error },
    #[snafu(display("Version {} not found locally", version))]
    VersionNotFound { version: UserVersion },
    #[snafu(display("{}", source))]
    CantUnderstandVersion { source: ApplicableVersionError },
}
