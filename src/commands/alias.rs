use super::command::Command;
use crate::alias::create_alias;
use crate::choose_version_for_user_input::{
    choose_version_for_user_input, Error as ApplicableVersionError,
};
use crate::config::FnmConfig;
use crate::user_version::UserVersion;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct Alias {
    pub(crate) to_version: UserVersion,
    pub(crate) name: String,
}

impl Command for Alias {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let applicable_version = choose_version_for_user_input(&self.to_version, config)
            .map_err(|source| Error::CantUnderstandVersion { source })?
            .ok_or(Error::VersionNotFound {
                version: self.to_version,
            })?;

        create_alias(config, &self.name, applicable_version.version())
            .map_err(|source| Error::CantCreateSymlink { source })?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't create symlink for alias: {}", source)]
    CantCreateSymlink { source: std::io::Error },
    #[error("Version {} not found locally", version)]
    VersionNotFound { version: UserVersion },
    #[error(transparent)]
    CantUnderstandVersion { source: ApplicableVersionError },
}
