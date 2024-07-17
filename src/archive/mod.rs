pub mod extract;
pub mod tar;
pub mod zip;

use std::io::Read;
use std::path::Path;

pub use self::extract::{Error, Extract};
#[cfg(unix)]
use self::tar::Tar;

#[cfg(windows)]
use self::zip::Zip;

pub enum Archive {
    #[cfg(windows)]
    Zip,
    #[cfg(unix)]
    TarXz,
    #[cfg(unix)]
    TarGz,
}

impl Archive {
    pub fn extract_archive_into(&self, path: &Path, response: impl Read) -> Result<(), Error> {
        let extractor: Box<dyn Extract> = match self {
            #[cfg(windows)]
            Self::Zip => Box::new(Zip::new(response)),
            #[cfg(unix)]
            Self::TarXz => Box::new(Tar::Xz(response)),
            #[cfg(unix)]
            Self::TarGz => Box::new(Tar::Gz(response)),
        };
        extractor.extract_into(path)?;
        Ok(())
    }

    pub fn get_file_suffix(&self) -> String {
        match self {
            #[cfg(windows)]
            Self::Zip => "zip",
            #[cfg(unix)]
            Self::TarXz => "tar.xz",
            #[cfg(unix)]
            Self::TarGz => "tar.gz",
        }.into()
    }

    #[cfg(windows)]
    pub fn supported() -> Vec<Self> {
        vec![Self::Zip]
    }

    #[cfg(unix)]
    pub fn supported() -> Vec<Self> {
        vec![Self::TarXz, Self::TarGz]
    }
}
