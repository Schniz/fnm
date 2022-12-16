use crate::config::FnmConfig;
use crate::remote_node_index;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct LsRemote {}

impl super::command::Command for LsRemote {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let all_versions = remote_node_index::list(&config.node_dist_mirror)?;

        for version in all_versions {
            print!("{}", version.version);
            if let Some(lts) = &version.lts {
                print!(" ({lts})");
            }
            println!();
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    HttpError {
        #[from]
        source: crate::http::Error,
    },
}
