use crate::version::Version;

#[derive(Clone, Debug)]
pub enum UserVersion {
    OnlyMajor(u64),
    MajorMinor(u64, u64),
    Full(Version),
}

impl UserVersion {
    pub fn to_version<'a, T>(&self, available_versions: T) -> Option<&'a Version>
    where
        T: IntoIterator<Item = &'a Version>,
    {
        available_versions.into_iter().filter(|x| &self == x).max()
    }

    pub fn alias_name(&self) -> Option<String> {
        match self {
            Self::Full(version) => version.alias_name(),
            _ => None,
        }
    }
}

impl PartialEq<Version> for UserVersion {
    fn eq(&self, other: &Version) -> bool {
        match (self, other) {
            (Self::Full(a), b) if a == b => true,
            (Self::Full(_), _) => false,
            (_, Version::Bypassed) => false,
            (_, Version::Lts(_)) => false,
            (_, Version::Alias(_)) => false,
            (Self::OnlyMajor(major), Version::Semver(other)) => *major == other.major,
            (Self::MajorMinor(major, minor), Version::Semver(other)) => {
                *major == other.major && *minor == other.minor
            }
        }
    }
}

fn next_of<'a, T: std::str::FromStr, It: Iterator<Item = &'a str>>(i: &mut It) -> Option<T> {
    let x = i.next()?;
    T::from_str(x).ok()
}

impl std::fmt::Display for UserVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Full(x) => x.fmt(f),
            Self::OnlyMajor(major) => write!(f, "v{}.x.x", major),
            Self::MajorMinor(major, minor) => write!(f, "v{}.{}.x", major, minor),
        }
    }
}

impl std::str::FromStr for UserVersion {
    type Err = semver::SemVerError;
    fn from_str(s: &str) -> Result<UserVersion, Self::Err> {
        match Version::parse(s) {
            Ok(v) => Ok(Self::Full(v)),
            Err(e) => {
                let mut parts = s.trim().split('.');
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
mod tests {
    use super::*;

    #[test]
    fn test_major_to_version() {
        let expected = Version::parse("6.1.0").unwrap();
        let versions = vec![
            Version::parse("6.0.0").unwrap(),
            Version::parse("6.0.1").unwrap(),
            expected.clone(),
            Version::parse("7.0.1").unwrap(),
        ];
        let result = UserVersion::OnlyMajor(6).to_version(&versions);

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
        let result = UserVersion::MajorMinor(6, 0).to_version(&versions);

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
        let result = UserVersion::Full(expected.clone()).to_version(&versions);

        assert_eq!(result, Some(&expected));
    }
}
