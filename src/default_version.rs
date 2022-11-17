use crate::config::FnmConfig;
use crate::version::Version;
use std::str::FromStr;

pub fn find_default_version(config: &FnmConfig) -> Option<Version> {
    let version_path = config.default_version_dir().canonicalize().ok()?;
    let file_name = version_path.parent()?.file_name()?;
    Version::from_str(file_name.to_str()?).ok()?.into()
}
