use crate::config::FnmConfig;
use crate::remote_node_index;
use crate::user_version::UserVersion;

use colored::Colorize;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct LsRemote {
    /// Filter with SemVer
    filter: Option<UserVersion>,

    /// Only show latest LTS versions
    #[arg(long)]
    lts: Option<Option<String>>,

    /// Version sorting order
    #[arg(long, default_value = "asc")]
    sort: remote_node_index::SortingMethod,
}

impl super::command::Command for LsRemote {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let mut all_versions = remote_node_index::list(&config.node_dist_mirror, &self.sort)?;

        if let Some(lts) = &self.lts {
            all_versions.retain(|v| match lts {
                None => v.lts.is_some(),
                Some(lts) => v
                    .lts
                    .as_ref()
                    .is_some_and(|this_lts| this_lts.eq_ignore_ascii_case(lts)),
            });
        }

        if let Some(filter) = &self.filter {
            all_versions.retain(|v| filter.matches(&v.version, config));
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
