use super::command::Command;
use crate::config::FnmConfig;
use crate::current_version::{current_version, Error};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Current {}

impl Command for Current {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let version_string = match current_version(&config)? {
            Some(ver) => ver.v_str(),
            None => "none".into(),
        };
        println!("{}", version_string);
        Ok(())
    }
}
