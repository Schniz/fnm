use crate::alias::{list_aliases, StoredAlias};
use crate::config::FnmConfig;
use crate::current_version::current_version;
use crate::outln;
use crate::version::Version;
use colored::*;
use snafu::{ResultExt, Snafu};
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct LsLocal {}

impl super::command::Command for LsLocal {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let base_dir = config.installations_dir();
        let mut versions: Vec<_> = std::fs::read_dir(&base_dir)
            .context(CantListLocallyInstalledVersion)?
            .filter_map(|x| {
                if let Ok(version_dir) = x {
                    let file_name = version_dir.file_name();
                    file_name.to_str().and_then(|x| Version::parse(x).ok())
                } else {
                    None
                }
            })
            .collect();
        versions.insert(0, Version::Bypassed);
        versions.sort();
        let aliases_hash = generate_aliases_hash(&config).context(CantReadAliases)?;
        let curr_version = current_version(&config).ok().flatten();

        for version in versions {
            let version_aliases = match aliases_hash.get(&version.v_str()) {
                None => "".into(),
                Some(versions) => {
                    let version_string = versions
                        .iter()
                        .map(|x| x.name())
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!(" {}", version_string.dimmed())
                }
            };

            let version_str = format!("* {}{}", version, version_aliases);

            if curr_version == Some(version) {
                outln!(config#Info, "{}", version_str.cyan());
            } else {
                outln!(config#Info, "{}", version_str);
            }
        }
        Ok(())
    }
}

fn generate_aliases_hash(config: &FnmConfig) -> std::io::Result<HashMap<String, Vec<StoredAlias>>> {
    let mut aliases = list_aliases(&config)?;
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

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Can't list locally installed versions: {}", source))]
    CantListLocallyInstalledVersion { source: std::io::Error },
    #[snafu(display("Can't read aliases: {}", source))]
    CantReadAliases { source: std::io::Error },
}
