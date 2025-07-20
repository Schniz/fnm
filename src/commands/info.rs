use crate::config::FnmConfig;
use crate::current_version::{current_version, Error as CurrentVersionError};
use crate::remote_node_index;
use crate::user_version::UserVersion;

use colored::Colorize;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct Info {
    /// Filter versions by a user-defined version or a semver range
    #[arg(long)]
    filter: Option<UserVersion>,

    /// Show only LTS versions (optionally filter by LTS codename)  
    #[arg(long)]
    #[expect(
        clippy::option_option,
        reason = "clap Option<Option<T>> supports --x and --x=value syntaxes"
    )]
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

impl super::command::Command for Info {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        // Get all versions from the Node.js URL.
        let all_versions = remote_node_index::list(&config.node_dist_mirror)
            .map_err(|source| Error::CantListRemoteVersions { source })?;

        // Filter the latest LTS version.
        let latest_lts = all_versions
            .iter()
            .rev()
            .find(|v| match &v.lts {
                Some(_) => true, // If it is Some, pass (any value)
                None => false,   // If it is None, do not pass
            })
            .map(|v| v.version.clone());

        // Filter the latest version "latest".
        let latest = all_versions.last().map(|v| v.version.clone());

        // Print the current version
        let version_string = match current_version(config)? {
            Some(ver) => ver.v_str(),
            None => "none".into(),
        };

        println!("Using: {}", format!("{version_string}").green());

        // Print the latest LTS version.
        if let Some(lts) = latest_lts {
            let version_str = format!("{lts}");

            println!("Current LTS: {}", version_str.cyan());
        } else {
            eprintln!("{}", "No LTS version found.".red());
        }

        // Print the latest version.
        if let Some(latest) = latest {
            let version_str = format!("{latest}");
            println!("Current latest: {}", version_str.cyan());
        } else {
            eprintln!("{}", "No latest version found.".red());
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    CurrentVersionError {
        #[from]
        source: CurrentVersionError,
    },
    #[error(transparent)]
    CantListRemoteVersions { source: remote_node_index::Error },
}
