use super::alias::Alias;
use super::command::Command;
use crate::alias::get_alias_by_name;
use crate::config::FnmConfig;
use crate::user_version::UserVersion;

#[derive(clap::Parser, Debug)]
pub struct Default {
    version: Option<UserVersion>,
}

impl Command for Default {
    type Error = super::alias::Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        match self.version {
            Some(version) => Alias {
                name: "default".into(),
                to_version: version,
            }
            .apply(config),
            None => match get_alias_by_name(config, "default") {
                Some(alias) => {
                    println!("{}", alias.s_ver());
                    Ok(())
                }
                None => Err(Self::Error::DefaultAliasDoesNotExist),
            },
        }
    }

    fn handle_error(err: Self::Error, config: &FnmConfig) {
        Alias::handle_error(err, config);
    }
}
