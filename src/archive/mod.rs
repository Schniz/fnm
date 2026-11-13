pub mod extract;
#[cfg(any(unix, debug_assertions))]
pub mod tar;
#[cfg(any(windows, debug_assertions))]
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
        match self {
            #[cfg(windows)]
            Self::Zip => Zip::new(response).extract_into(path),
            #[cfg(unix)]
            Self::TarXz => Tar::Xz(response).extract_into(path),
            #[cfg(unix)]
            Self::TarGz => Tar::Gz(response).extract_into(path),
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            #[cfg(windows)]
            Self::Zip => "zip",
            #[cfg(unix)]
            Self::TarXz => "tar.xz",
            #[cfg(unix)]
            Self::TarGz => "tar.gz",
        }
    }

    #[cfg(windows)]
    pub fn supported() -> &'static [Self] {
        &[Self::Zip]
    }

    #[cfg(unix)]
    pub fn supported() -> &'static [Self] {
        &[Self::TarXz, Self::TarGz]
    }
}
