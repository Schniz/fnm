use super::alias::Alias;
use super::command::Command;
use crate::config::FnmConfig;
use crate::user_version::UserVersion;

#[derive(clap::Parser, Debug)]
pub struct Default {
    version: UserVersion,
}

impl Command for Default {
    type Error = super::alias::Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        Alias {
            name: "default".into(),
            to_version: self.version,
        }
        .apply(config)
    }

    fn handle_error(err: Self::Error, config: &FnmConfig) {
        Alias::handle_error(err, config);
    }
}
