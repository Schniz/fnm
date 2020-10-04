use crate::version::Version;
use snafu::{ResultExt, Snafu};
use std::path::Path;

pub fn list<P: AsRef<Path>>(installations_dir: P) -> Result<Vec<Version>, Error> {
    let mut vec = vec![];
    for result_entry in installations_dir.as_ref().read_dir().context(IoError)? {
        let entry = result_entry.context(IoError)?;
        let path = entry.path();
        let filename = path
            .file_name()
            .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))
            .context(IoError)?
            .to_str()
            .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))
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
