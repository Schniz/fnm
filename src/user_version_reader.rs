use crate::config::FnmConfig;
use crate::user_version::UserVersion;
use crate::version_files::{get_user_version_for_directory, get_user_version_for_file};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum UserVersionReader {
    Direct(UserVersion),
    Path(PathBuf),
}

impl UserVersionReader {
    pub fn into_user_version(self, config: &FnmConfig) -> Option<UserVersion> {
        match self {
            Self::Direct(uv) => Some(uv),
            Self::Path(pathbuf) if pathbuf.is_file() => get_user_version_for_file(pathbuf, config),
            Self::Path(pathbuf) => get_user_version_for_directory(pathbuf, config),
        }
    }
}

impl FromStr for UserVersionReader {
    type Err = node_semver::SemverError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pathbuf = PathBuf::from_str(s);
        let user_version = UserVersion::from_str(s);
        match (user_version, pathbuf) {
            (_, Ok(pathbuf)) if pathbuf.exists() => Ok(Self::Path(pathbuf)),
            (Ok(user_version), _) => Ok(Self::Direct(user_version)),
            (Err(user_version_err), _) => Err(user_version_err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::version::Version;
    use pretty_assertions::assert_eq;
    use std::io::Write;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_file_pathbuf_to_version() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"14").unwrap();
        let pathbuf = file.path().to_path_buf();

        let user_version =
            UserVersionReader::Path(pathbuf).into_user_version(&FnmConfig::default());
        assert_eq!(user_version, Some(UserVersion::OnlyMajor(14)));
    }

    #[test]
    fn test_directory_pathbuf_to_version() {
        let directory = TempDir::new().unwrap();
        let node_version_path = directory.path().join(".node-version");
        std::fs::write(node_version_path, "14").unwrap();
        let pathbuf = directory.path().to_path_buf();

        let user_version =
            UserVersionReader::Path(pathbuf).into_user_version(&FnmConfig::default());
        assert_eq!(user_version, Some(UserVersion::OnlyMajor(14)));
    }

    #[test]
    fn test_direct_to_version() {
        let user_version = UserVersionReader::Direct(UserVersion::OnlyMajor(14))
            .into_user_version(&FnmConfig::default());
        assert_eq!(user_version, Some(UserVersion::OnlyMajor(14)));
    }

    #[test]
    fn test_from_str_directory() {
        let directory = TempDir::new().unwrap();
        let node_version_path = directory.path().join(".node-version");
        std::fs::write(node_version_path, "14").unwrap();
        let pathbuf = directory.path().to_path_buf();

        let user_version = UserVersionReader::from_str(pathbuf.to_str().unwrap());
        assert!(matches!(user_version, Ok(UserVersionReader::Path(_))));
    }

    #[test]
    fn test_from_str_file() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "14").unwrap();
        let pathbuf = file.path().to_path_buf();

        let user_version = UserVersionReader::from_str(pathbuf.to_str().unwrap());
        assert!(matches!(user_version, Ok(UserVersionReader::Path(_))));
    }

    #[test]
    fn test_non_existing_path() {
        let user_version =
            UserVersionReader::from_str("/tmp/some_random_text_that_probably_does_not_exist");
        assert!(matches!(
            user_version,
            Ok(UserVersionReader::Direct(UserVersion::Full(
                Version::Alias(_)
            )))
        ));
    }

    #[test]
    fn test_a_version_number() {
        let user_version = UserVersionReader::from_str("12.0");
        assert!(matches!(
            user_version,
            Ok(UserVersionReader::Direct(UserVersion::MajorMinor(12, 0)))
        ));
    }
}
