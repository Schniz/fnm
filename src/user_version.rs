use crate::version::Version;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum UserVersion {
    Node,
    OnlyMajor(u64),
    MajorMinor(u64, u64),
    SemverRange(node_semver::Range),
    Full(Version),
}

impl UserVersion {
    /// Let a stored `node` alias override the built-in version request.
    pub fn resolve_alias(self, config: &crate::config::FnmConfig) -> Self {
        match self {
            Self::Node => match config.aliases_dir().join("node").symlink_metadata() {
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => Self::Node,
                _ => Self::Full(Version::Alias("node".into())),
            },
            version => version,
        }
    }

    pub fn to_version<'a, T>(
        &self,
        available_versions: T,
        config: &crate::config::FnmConfig,
    ) -> Option<&'a Version>
    where
        T: IntoIterator<Item = &'a Version>,
    {
        available_versions
            .into_iter()
            .filter(|x| self.matches(x, config))
            .max()
    }

    pub fn alias_name(&self) -> Option<String> {
        match self {
            Self::Full(version) => version.alias_name(),
            _ => None,
        }
    }

    pub fn matches(&self, version: &Version, config: &crate::config::FnmConfig) -> bool {
        match (self, version) {
            (Self::Node, Version::Semver(semver)) => !semver.is_prerelease(),
            (Self::Full(a), b) if a == b => true,
            (Self::Full(user_version), maybe_alias) => {
                match (user_version.alias_name(), maybe_alias.find_aliases(config)) {
                    (None, _) | (_, Err(_)) => false,
                    (Some(user_alias), Ok(aliases)) => {
                        aliases.iter().any(|alias| alias.name() == user_alias)
                    }
                }
            }
            (Self::SemverRange(range), Version::Semver(semver)) => semver.satisfies(range),
            (_, Version::Bypassed | Version::Lts(_) | Version::Alias(_) | Version::Latest) => false,
            (Self::OnlyMajor(major), Version::Semver(other)) => *major == other.major,
            (Self::MajorMinor(major, minor), Version::Semver(other)) => {
                *major == other.major && *minor == other.minor
            }
        }
    }

    /// The inferred alias for the user version, if it exists.
    pub fn inferred_alias(&self) -> Option<Version> {
        match self {
            UserVersion::Full(Version::Latest) => Some(Version::Latest),
            UserVersion::Full(Version::Lts(lts_type)) => Some(Version::Lts(lts_type.clone())),
            _ => None,
        }
    }
}

fn next_of<'a, T: FromStr, It: Iterator<Item = &'a str>>(i: &mut It) -> Option<T> {
    let x = i.next()?;
    T::from_str(x).ok()
}

impl std::fmt::Display for UserVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Node => write!(f, "node"),
            Self::Full(x) => x.fmt(f),
            Self::SemverRange(x) => x.fmt(f),
            Self::OnlyMajor(major) => write!(f, "v{major}.x.x"),
            Self::MajorMinor(major, minor) => write!(f, "v{major}.{minor}.x"),
        }
    }
}

fn skip_first_v(str: &str) -> &str {
    str.strip_prefix('v').unwrap_or(str)
}

impl FromStr for UserVersion {
    type Err = node_semver::SemverError;
    fn from_str(s: &str) -> Result<UserVersion, Self::Err> {
        match Version::parse(s) {
            Ok(Version::Alias(alias)) if alias == "node" => Ok(Self::Node),
            Ok(v) => Ok(Self::Full(v)),
            Err(e) => {
                let mut parts = skip_first_v(s.trim()).split('.');
                match (next_of::<u64, _>(&mut parts), next_of::<u64, _>(&mut parts)) {
                    (Some(major), None) => Ok(Self::OnlyMajor(major)),
                    (Some(major), Some(minor)) => Ok(Self::MajorMinor(major, minor)),
                    _ => Err(e),
                }
            }
        }
    }
}

#[cfg(test)]
impl PartialEq for UserVersion {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Node, Self::Node) => true,
            (Self::OnlyMajor(a), Self::OnlyMajor(b)) if a == b => true,
            (Self::MajorMinor(a1, a2), Self::MajorMinor(b1, b2)) if (a1, a2) == (b1, b2) => true,
            (Self::Full(v1), Self::Full(v2)) if v1 == v2 => true,
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::FnmConfig;

    use super::*;

    #[test]
    fn test_node_request() {
        for input in ["node", "NODE", "Node"] {
            assert_eq!(UserVersion::from_str(input).unwrap(), UserVersion::Node);
        }
        assert_eq!(UserVersion::Node.to_string(), "node");
        assert_eq!(
            Version::parse("node").unwrap(),
            Version::Alias("node".into())
        );
    }

    #[test]
    fn test_node_selects_latest_stable_version() {
        let expected = Version::parse("20.14.0").unwrap();
        let versions = vec![
            expected.clone(),
            Version::parse("18.20.3").unwrap(),
            Version::parse("21.0.0-rc.1").unwrap(),
            Version::Bypassed,
            Version::Latest,
            Version::Alias("custom".into()),
            Version::Lts(crate::lts::LtsType::Latest),
        ];
        assert_eq!(
            UserVersion::Node.to_version(&versions, &FnmConfig::default()),
            Some(&expected)
        );
        assert_eq!(
            UserVersion::Node.to_version(&[], &FnmConfig::default()),
            None
        );
    }

    #[test]
    fn test_node_preserves_stored_alias() {
        let directory = tempfile::TempDir::new().unwrap();
        let config = FnmConfig::default().with_base_dir(Some(directory.path().into()));
        assert_eq!(UserVersion::Node.resolve_alias(&config), UserVersion::Node);

        let expected = Version::parse("18.20.3").unwrap();
        std::fs::create_dir_all(expected.installation_path(&config)).unwrap();
        crate::alias::create_alias(&config, "node", &expected).unwrap();

        let request = UserVersion::Node.resolve_alias(&config);
        assert_eq!(request, UserVersion::Full(Version::Alias("node".into())));
        let versions = vec![expected.clone(), Version::parse("20.14.0").unwrap()];
        assert_eq!(request.to_version(&versions, &config), Some(&expected));

        std::fs::remove_dir_all(expected.installation_path(&config)).unwrap();
        assert_eq!(
            UserVersion::Node.resolve_alias(&config),
            UserVersion::Full(Version::Alias("node".into()))
        );
    }

    #[test]
    fn test_parsing_only_major() {
        let version = UserVersion::from_str("10").ok();
        assert_eq!(version, Some(UserVersion::OnlyMajor(10)));
    }

    #[test]
    fn test_parsing_major_minor() {
        let version = UserVersion::from_str("10.20").ok();
        assert_eq!(version, Some(UserVersion::MajorMinor(10, 20)));
    }

    #[test]
    fn test_parsing_only_major_with_v() {
        let version = UserVersion::from_str("v10").ok();
        assert_eq!(version, Some(UserVersion::OnlyMajor(10)));
    }

    #[test]
    fn test_major_to_version() {
        let expected = Version::parse("6.1.0").unwrap();
        let versions = vec![
            Version::parse("6.0.0").unwrap(),
            Version::parse("6.0.1").unwrap(),
            expected.clone(),
            Version::parse("7.0.1").unwrap(),
        ];
        let result = UserVersion::OnlyMajor(6).to_version(&versions, &FnmConfig::default());

        assert_eq!(result, Some(&expected));
    }

    #[test]
    fn test_major_minor_to_version() {
        let expected = Version::parse("6.0.1").unwrap();
        let versions = vec![
            Version::parse("6.0.0").unwrap(),
            Version::parse("6.1.0").unwrap(),
            expected.clone(),
            Version::parse("7.0.1").unwrap(),
        ];
        let result = UserVersion::MajorMinor(6, 0).to_version(&versions, &FnmConfig::default());

        assert_eq!(result, Some(&expected));
    }

    #[test]
    fn test_semver_to_version() {
        let expected = Version::parse("6.0.0").unwrap();
        let versions = vec![
            expected.clone(),
            Version::parse("6.1.0").unwrap(),
            Version::parse("6.0.1").unwrap(),
            Version::parse("7.0.1").unwrap(),
        ];
        let result =
            UserVersion::Full(expected.clone()).to_version(&versions, &FnmConfig::default());

        assert_eq!(result, Some(&expected));
    }
}
