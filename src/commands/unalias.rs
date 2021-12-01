use super::command::Command;
use crate::fs::remove_symlink_dir;
use crate::user_version::UserVersion;
use crate::version::Version;
use crate::{choose_version_for_user_input, config::FnmConfig};
use snafu::{OptionExt, ResultExt, Snafu};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Unalias {
    pub(crate) requested_alias: String,
}

impl Command for Unalias {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let requested_version = choose_version_for_user_input::choose_version_for_user_input(
            &UserVersion::Full(Version::Alias(self.requested_alias.clone())),
            config,
        )
        .ok()
        .flatten()
        .with_context(|| AliasNotFound {
            requested_alias: self.requested_alias,
        })?;

        remove_symlink_dir(&requested_version.path()).context(CantDeleteSymlink)?;

        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Can't delete symlink: {}", source))]
    CantDeleteSymlink { source: std::io::Error },
    #[snafu(display("Requested alias {} not found", requested_alias))]
    AliasNotFound { requested_alias: String },
}
