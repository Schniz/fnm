use crate::version::Version;
use snafu::{ResultExt, Snafu};
use std::path::Path;

pub fn list<P: AsRef<Path>>(installations_dir: P) -> Result<Vec<Version>, Error> {
    let mut vec = vec![];
    for result_entry in installations_dir.as_ref().read_dir().context(IoError)? {
        let entry = result_entry.context(IoError)?;
        if entry.file_name() == ".downloads" {
            continue;
        }

        let path = entry.path();
        let filename = path
            .file_name()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
            .context(IoError)?
            .to_str()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
            .context(IoError)?;
        let version = Version::parse(filename).context(SemverError)?;
        vec.push(version);
    }
    Ok(vec)
}

#[derive(Debug, Snafu)]
pub enum Error {
    IoError { source: std::io::Error },
    SemverError { source: semver::SemVerError },
}
