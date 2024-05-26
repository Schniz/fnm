pub mod extract;
pub mod tar_xz;
pub mod zip;

pub use self::extract::{Error, Extract};

#[cfg(unix)]
pub use self::tar_xz::TarXz;

#[cfg(windows)]
pub use self::zip::Zip;
