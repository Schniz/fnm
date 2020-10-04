use crate::config::FnmConfig;
use crate::remote_node_index;
use snafu::{ResultExt, Snafu};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct LsRemote {}

impl super::command::Command for LsRemote {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let mut all_versions =
            remote_node_index::list(&config.node_dist_mirror).context(HttpError)?;
        all_versions.sort();

        for version in all_versions {
            print!("{}", version.version);
            if let Some(lts) = &version.lts {
                print!(" ({})", lts);
            }
            println!("");
        }

        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    HttpError { source: reqwest::Error },
}
