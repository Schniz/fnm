use crate::config::FnmConfig;
use crate::remote_node_index;
use crate::user_version::UserVersion;

use colored::Colorize;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct LsRemote {
    /// Filter for versions
    filter: Option<UserVersion>,

    // Version sorting order
    #[arg(long, default_value = "asc")]
    sort: remote_node_index::SortingMethod,
}

impl super::command::Command for LsRemote {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let mut all_versions = remote_node_index::list(&config.node_dist_mirror, &self.sort)?;

        if let Some(filter) = &self.filter {
            all_versions = all_versions
                .into_iter()
                .filter(|v| filter.matches(&v.version, config))
                .collect();
        }

        for version in &all_versions {
            print!("{}", version.version);
            if let Some(lts) = &version.lts {
                print!("{}", format!(" ({lts})").cyan());
            }
            println!();
        }

        if all_versions.is_empty() {
            eprintln!("{}", "No versions were found!".red());
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
