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
    fn extract_into(self, path: &Path) -> Result<(), Error> {
        self.extract_into_impl(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_tar_gz_extraction() {
        let temp_dir = &tempfile::tempdir().expect("Can't create a temp directory");
        let response =
            crate::http::get("https://nodejs.org/dist/v12.0.0/node-v12.0.0-darwin-x64.tar.gz")
                .and_then(reqwest::blocking::Response::error_for_status)
                .expect("Can't make request to Node v12.0.0 zip file");
        Tar::Gz(response)
            .extract_into(temp_dir.as_ref())
            .expect("Can't unzip files");
        dbg!(&temp_dir);
        let node_file = temp_dir
            .as_ref()
            .join("node-v12.0.0-darwin-x64")
            .join("bin")
            .join("node");
        assert!(node_file.exists());
    }

    #[test_log::test]
    fn test_tar_xz_extraction() {
        let temp_dir = &tempfile::tempdir().expect("Can't create a temp directory");
        let response =
            crate::http::get("https://nodejs.org/dist/v12.0.0/node-v12.0.0-darwin-x64.tar.xz")
                .and_then(reqwest::blocking::Response::error_for_status)
                .expect("Can't make request to Node v12.0.0 zip file");
        Tar::Xz(response)
            .extract_into(temp_dir.as_ref())
            .expect("Can't unzip files");
        dbg!(&temp_dir);
        let node_file = temp_dir
            .as_ref()
            .join("node-v12.0.0-darwin-x64")
            .join("bin")
            .join("node");
        assert!(node_file.exists());
    }
}
