use std::error::Error as StdError;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ZipError(zip::result::ZipError),
    HttpError(ureq::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(x) => x.fmt(f),
            Self::ZipError(x) => x.fmt(f),
            Self::HttpError(x) => x.fmt(f),
        }
    }
}

impl StdError for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Self {
        Self::ZipError(err)
    }
}

impl From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        Self::HttpError(err)
    }
}

pub trait Extract {
    fn extract_into<P: AsRef<Path>>(self, path: P) -> Result<(), Error>;
}
