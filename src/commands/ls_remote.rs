use crate::config::FnmConfig;
use crate::remote_node_index;
use crate::user_version::UserVersion;

use colored::Colorize;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct LsRemote {
    /// Filter versions by a user-defined version or a semver range
    #[arg(long)]
    filter: Option<UserVersion>,

    /// Show only LTS versions (optionally filter by LTS codename)  
    #[arg(long)]
    #[allow(clippy::option_option)]
    lts: Option<Option<String>>,

    /// Version sorting order
    #[arg(long, default_value = "asc")]
    sort: SortingMethod,

    /// Only show the latest matching version
    #[arg(long)]
    latest: bool,
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
pub enum SortingMethod {
    #[clap(name = "desc")]
    /// Sort versions in descending order (latest to earliest)
    Descending,
    #[clap(name = "asc")]
    /// Sort versions in ascending order (earliest to latest)
    Ascending,
}

/// Drain all elements but the last one
fn truncate_except_latest<T>(list: &mut Vec<T>) {
    let len = list.len();
    if len > 1 {
        list.swap(0, len - 1);
        list.truncate(1);
    }
}

impl super::command::Command for LsRemote {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let mut all_versions = remote_node_index::list(&config.node_dist_mirror)?;

        if let Some(lts) = &self.lts {
            match lts {
                Some(codename) => all_versions.retain(|v| {
                    v.lts
                        .as_ref()
                        .is_some_and(|v_lts| v_lts.eq_ignore_ascii_case(codename))
                }),
                None => all_versions.retain(|v| v.lts.is_some()),
            };
        }

        if let Some(filter) = &self.filter {
            all_versions.retain(|v| filter.matches(&v.version, config));
        }

        if self.latest {
            truncate_except_latest(&mut all_versions);
        }

        if let SortingMethod::Descending = self.sort {
            all_versions.reverse();
        }

        if all_versions.is_empty() {
            eprintln!("{}", "No versions were found!".red());
            return Ok(());
        }

        for version in &all_versions {
            print!("{}", version.version);
            if let Some(lts) = &version.lts {
                print!("{}", format!(" ({lts})").cyan());
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_truncate_except_latest() {
        let mut list = vec![1, 2, 3, 4, 5];
        truncate_except_latest(&mut list);
        assert_eq!(list, vec![5]);

        let mut list: Vec<()> = vec![];
        truncate_except_latest(&mut list);
        assert_eq!(list, vec![]);

        let mut list = vec![1];
        truncate_except_latest(&mut list);
        assert_eq!(list, vec![1]);
    }
}
