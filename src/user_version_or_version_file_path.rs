use crate::user_version::UserVersion;
use crate::version_files::{get_user_version_for_directory, get_user_version_for_file};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub enum UserVersionOrVersionFilePath {
    UserVersion(UserVersion),
    File(PathBuf),
    Directory(PathBuf),
}

impl UserVersionOrVersionFilePath {
    pub fn to_user_version(self) -> Option<UserVersion> {
        match self {
            Self::UserVersion(uv) => Some(uv),
            Self::File(pathbuf) => get_user_version_for_file(&pathbuf),
            Self::Directory(pathbuf) => get_user_version_for_directory(&pathbuf),
        }
    }
}

impl FromStr for UserVersionOrVersionFilePath {
    type Err = semver::SemVerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pathbuf = PathBuf::from_str(&s);
        let user_version = UserVersion::from_str(&s);
        match (user_version, pathbuf) {
            (_, Ok(pathbuf)) if pathbuf.exists() => {
                if pathbuf.is_dir() {
                    Ok(Self::Directory(pathbuf))
                } else {
                    Ok(Self::File(pathbuf))
                }
            }
            (Ok(user_version), _) => Ok(Self::UserVersion(user_version)),
            (Err(user_version_err), _) => Err(user_version_err),
        }
    }
}
