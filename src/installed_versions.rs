use crate::version::Version;
use std::path::Path;
use thiserror::Error;

pub fn list<P: AsRef<Path>>(installations_dir: P) -> Result<Vec<Version>, Error> {
    let mut vec = vec![];
    for result_entry in installations_dir.as_ref().read_dir()? {
        let entry = result_entry?;
        if entry
            .file_name()
            .to_str()
            .map_or(false, |s| s.starts_with('.'))
        {
            continue;
        }

        let path = entry.path();
        let filename = path
            .file_name()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?
            .to_str()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;
        let version = Version::parse(filename)?;
        vec.push(version);
    }
    Ok(vec)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error(transparent)]
    SemverError {
        #[from]
        source: node_semver::SemverError,
    },
}
