use super::extract::{Error, Extract};
use std::{io::Read, path::Path};

pub struct TarXz<R: Read> {
    response: R,
}

impl<R: Read> TarXz<R> {
    #[allow(dead_code)]
    pub fn new(response: R) -> Self {
        Self { response }
    }
}

impl<R: Read> Extract for TarXz<R> {
    fn extract_into<P: AsRef<Path>>(self, path: P) -> Result<(), Error> {
        let xz_stream = xz2::read::XzDecoder::new(self.response);
        let mut tar_archive = tar::Archive::new(xz_stream);
        tar_archive.unpack(&path)?;
        Ok(())
    }
}
