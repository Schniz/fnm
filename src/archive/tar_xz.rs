use super::extract::{Error, Extract};
use reqwest::blocking::Response;
use std::path::Path;

pub struct TarXz {
    response: Response,
}

impl TarXz {
    #[allow(dead_code)]
    pub fn new(response: Response) -> Self {
        Self { response }
    }
}

impl Extract for TarXz {
    fn extract_into<P: AsRef<Path>>(self, path: P) -> Result<(), Error> {
        let xz_stream = xz2::read::XzDecoder::new(self.response);
        let mut tar_archive = tar::Archive::new(xz_stream);
        tar_archive.unpack(&path)?;
        Ok(())
    }
}
