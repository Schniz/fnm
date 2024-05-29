use crate::alias;
use crate::config;
use crate::lts::LtsType;
use crate::system_version;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Version {
    Semver(node_semver::Version),
    Lts(LtsType),
    Alias(String),
    Latest,
    Bypassed,
}

fn first_letter_is_number(s: &str) -> bool {
    s.chars().next().map_or(false, |x| x.is_ascii_digit())
}

impl Version {
    pub fn parse<S: AsRef<str>>(version_str: S) -> Result<Self, node_semver::SemverError> {
        let lowercased = version_str.as_ref().to_lowercase();
        if lowercased == system_version::display_name() {
            Ok(Self::Bypassed)
        } else if lowercased.starts_with("lts-") || lowercased.starts_with("lts/") {
            let lts_type = LtsType::from(&lowercased[4..]);
            Ok(Self::Lts(lts_type))
        } else if first_letter_is_number(lowercased.trim_start_matches('v')) {
            let version_plain = lowercased.trim_start_matches('v');
            let sver = node_semver::Version::parse(version_plain)?;
            Ok(Self::Semver(sver))
        } else {
            Ok(Self::Alias(lowercased))
        }
    }

    pub fn alias_name(&self) -> Option<String> {
        match self {
            l @ (Self::Lts(_) | Self::Alias(_)) => Some(l.v_str()),
            _ => None,
        }
    }

    pub fn find_aliases(
        &self,
        config: &config::FnmConfig,
    ) -> std::io::Result<Vec<alias::StoredAlias>> {
        let aliases = alias::list_aliases(config)?
            .drain(..)
            .filter(|alias| alias.s_ver() == self.v_str())
            .collect();
        Ok(aliases)
    }

    pub fn v_str(&self) -> String {
        format!("{self}")
    }

    pub fn installation_path(&self, config: &config::FnmConfig) -> std::path::PathBuf {
        match self {
            Self::Bypassed => system_version::path(),
            v @ (Self::Lts(_) | Self::Alias(_) | Self::Latest) => {
                config.aliases_dir().join(v.alias_name().unwrap())
            }
            v @ Self::Semver(_) => config
                .installations_dir()
                .join(v.v_str())
                .join("installation"),
        }
    }

    pub fn root_path(&self, config: &config::FnmConfig) -> Option<std::path::PathBuf> {
        let path = self.installation_path(config);
        let mut canon_path = path.canonicalize().ok()?;
        canon_path.pop();
        Some(canon_path)
    }
}

// TODO: add a trait called BinPath that &Path and PathBuf implements
// which adds the `.bin_path()` which works both on windows and unix :)

impl<'de> serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version_str = String::deserialize(deserializer)?;
        Version::parse(version_str).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bypassed => write!(f, "{}", system_version::display_name()),
            Self::Lts(lts) => write!(f, "lts-{lts}"),
            Self::Semver(semver) => write!(f, "v{semver}"),
            Self::Alias(alias) => write!(f, "{alias}"),
            Self::Latest => write!(f, "latest"),
        }
    }
}

impl FromStr for Version {
    type Err = node_semver::SemverError;
    fn from_str(s: &str) -> Result<Version, Self::Err> {
        Self::parse(s)
    }
}

impl PartialEq<node_semver::Version> for Version {
    fn eq(&self, other: &node_semver::Version) -> bool {
        match self {
            Self::Bypassed | Self::Lts(_) | Self::Alias(_) | Self::Latest => false,
            Self::Semver(v) => v == other,
        }
    }
}
