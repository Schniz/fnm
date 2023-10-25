use crate::config::FnmConfig;
use crate::remote_node_index;
use colored::Colorize;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct LsRemote {
    /// Filter for version prefixes
    filter: Option<String>,

    // Version sorting order
    #[arg(long, default_value = "asc")]
    sort: remote_node_index::SortingMethod,
}

impl super::command::Command for LsRemote {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let filter = self
            .filter
            .and_then(|f| Some(f.strip_prefix('v').unwrap_or(&f).to_owned()));

        let mut all_versions = remote_node_index::list(&config.node_dist_mirror, &self.sort)?;

        if let Some(filter) = &filter {
            all_versions = all_versions
                .into_iter()
                .filter(|v| {
                    v.version
                        .v_str()
                        .strip_prefix('v')
                        .and_then(|rv| Some(rv.starts_with(filter)))
                        .unwrap_or(false)
                })
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
            eprintln!(
                "{}",
                format!(
                    "No versions were found{}!",
                    match filter {
                        Some(filter) => format!(" with prefix {filter}"),
                        None => "".to_owned(),
                    }
                )
                .red()
            );
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
