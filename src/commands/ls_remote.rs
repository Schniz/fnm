use crate::config::FnmConfig;
use crate::remote_node_index::{self, IndexedNodeVersion};
use thiserror::Error;

#[derive(Debug)]
enum LtsOption {
    Bool(bool),
    String(String),
}

impl std::str::FromStr for LtsOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LtsOption::String(s.to_owned()))
    }
}

#[derive(clap::Parser, Debug)]
pub struct LsRemote {
    /// List lts version.
    /// use --lts to list all lts version.
    /// use --lts=hydrogen to filter lts version.
    #[clap(long)]
    lts: Option<Option<LtsOption>>,
}

impl LsRemote {
    fn run(self, config: &FnmConfig) -> Result<Vec<IndexedNodeVersion>, Error> {
        let all_versions = remote_node_index::list(&config.node_dist_mirror)?;

        let (lts, lts_name) = match self
            .lts
            .unwrap_or(Some(LtsOption::Bool(false)))
            .unwrap_or(LtsOption::Bool(true))
        {
            LtsOption::Bool(lts) => (lts, String::from("")),
            LtsOption::String(lts_name) => (true, lts_name.trim().to_owned()),
        };

        let avaliable_versions = if lts {
            all_versions
                .into_iter()
                .filter(|version| version.lts.is_some())
                .filter(|version| {
                    lts_name.is_empty()
                        || version
                            .lts
                            .as_deref()
                            .unwrap_or_default()
                            .to_lowercase()
                            .contains(&lts_name.to_lowercase())
                })
                .collect()
        } else {
            all_versions
        };

        for version in &avaliable_versions {
            print!("{}", version.version);
            if let Some(lts) = &version.lts {
                print!(" ({lts})");
            }
            println!();
        }

        Ok(avaliable_versions)
    }
}

impl super::command::Command for LsRemote {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        self.run(&config)?;

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
mod tests {
    use super::*;

    #[test]
    fn test_ls_remote() {
        let config = FnmConfig::default();

        let all_versions = remote_node_index::list(&config.node_dist_mirror).unwrap();
        let versions = LsRemote { lts: None }.run(&config).unwrap();

        assert!(all_versions.len() == versions.len());
    }

    #[test]
    fn test_ls_remote_with_lts() {
        let config = FnmConfig::default();

        let versions = LsRemote { lts: Some(None) }.run(&config).unwrap();

        assert!(versions.iter().all(|version| version.lts.is_some()));
    }

    #[test]
    fn test_ls_remote_with_lts_and_filter() {
        let config = FnmConfig::default();
        let filter_name = "drogen";

        let versions = LsRemote {
            lts: Some(Some(LtsOption::String(filter_name.to_owned()))),
        }
        .run(&config)
        .unwrap();

        assert!(versions.iter().all(|version| {
            version.lts.is_some() && version.lts.as_deref().unwrap().contains(filter_name)
        }));
    }
}
