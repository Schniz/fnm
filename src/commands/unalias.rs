use super::command::Command;
use crate::config::FnmConfig;
use crate::fs::remove_symlink_dir;
use snafu::{ensure, ResultExt, Snafu};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Unalias {
    pub(crate) requested_alias: String,
}

impl Command for Unalias {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let alias_path = config.aliases_dir().join(&self.requested_alias);
        ensure!(
            alias_path.exists(),
            AliasNotFound {
                requested_alias: self.requested_alias
            }
        );

        remove_symlink_dir(&alias_path).context(CantDeleteSymlink)?;

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
