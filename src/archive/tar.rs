use super::{extract::Error, Extract};
use std::{io::Read, path::Path};

pub enum Tar<R: Read> {
    /// Tar archive with XZ compression
    Xz(R),
    /// Tar archive with Gzip compression
    Gz(R),
}

impl<R: Read> Tar<R> {
    fn extract_into_impl<P: AsRef<Path>>(self, path: P) -> Result<(), Error> {
        let stream: Box<dyn Read> = match self {
            Self::Xz(response) => Box::new(xz2::read::XzDecoder::new(response)),
            Self::Gz(response) => Box::new(flate2::read::GzDecoder::new(response)),
        };
        let mut tar_archive = tar::Archive::new(stream);
        tar_archive.unpack(&path)?;
        Ok(())
    }
}

impl<R: Read> Extract for Tar<R> {
    fn extract_into(self: Box<Self>, path: &Path) -> Result<(), Error> {
        self.extract_into_impl(path)
    }
}
