use crate::alias::{list_aliases, StoredAlias};
use crate::config::FnmConfig;
use crate::current_version::current_version;
use crate::version::Version;
use colored::Colorize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct LsLocal {}

impl super::command::Command for LsLocal {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let base_dir = config.installations_dir();
        let mut versions = crate::installed_versions::list(base_dir)
            .map_err(|source| Error::CantListLocallyInstalledVersion { source })?;
        versions.insert(0, Version::Bypassed);
        versions.sort();
        let aliases_hash =
            generate_aliases_hash(config).map_err(|source| Error::CantReadAliases { source })?;
        let curr_version = current_version(config).ok().flatten();

        for version in versions {
            let version_aliases = match aliases_hash.get(&version.v_str()) {
                None => String::new(),
                Some(versions) => {
                    let version_string = versions
                        .iter()
                        .map(StoredAlias::name)
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!(" {}", version_string.dimmed())
                }
            };

            let version_str = format!("* {version}{version_aliases}");

            if curr_version == Some(version) {
                println!("{}", version_str.cyan());
            } else {
                println!("{version_str}");
            }
        }
        Ok(())
    }
}

fn generate_aliases_hash(config: &FnmConfig) -> std::io::Result<HashMap<String, Vec<StoredAlias>>> {
    let mut aliases = list_aliases(config)?;
    let mut hashmap: HashMap<String, Vec<StoredAlias>> = HashMap::with_capacity(aliases.len());
    for alias in aliases.drain(..) {
        if let Some(value) = hashmap.get_mut(alias.s_ver()) {
            value.push(alias);
        } else {
            hashmap.insert(alias.s_ver().into(), vec![alias]);
        }
    }
    Ok(hashmap)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't list locally installed versions: {}", source)]
    CantListLocallyInstalledVersion {
        source: crate::installed_versions::Error,
    },
    #[error("Can't read aliases: {}", source)]
    CantReadAliases { source: std::io::Error },
}
