use crate::config::FnmConfig;
use crate::default_version;
use crate::user_version::UserVersion;
use crate::version_file_strategy::VersionFileStrategy;
use encoding_rs_io::DecodeReaderBytes;
use log::info;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

const PATH_PARTS: [&str; 2] = [".nvmrc", ".node-version"];

pub fn get_user_version_for_directory(
    path: impl AsRef<Path>,
    config: &FnmConfig,
) -> Option<UserVersion> {
    match config.version_file_strategy() {
        VersionFileStrategy::Local => get_user_version_for_single_directory(path),
        VersionFileStrategy::Recursive => {
            get_user_version_for_directory_recursive(path).or_else(|| {
                info!("Did not find anything recursively. Falling back to default alias.");
                default_version::find_default_version(config).map(UserVersion::Full)
            })
        }
    }
}

fn get_user_version_for_directory_recursive(path: impl AsRef<Path>) -> Option<UserVersion> {
    let mut current_path = Some(path.as_ref());

    while let Some(child_path) = current_path {
        if let Some(version) = get_user_version_for_single_directory(child_path) {
            return Some(version);
        }

        current_path = child_path.parent();
    }

    None
}

pub fn get_user_version_for_single_directory(path: impl AsRef<Path>) -> Option<UserVersion> {
    let path = path.as_ref();

    for path_part in &PATH_PARTS {
        let new_path = path.join(path_part);
        info!(
            "Looking for version file in {}. exists? {}",
            new_path.display(),
            new_path.exists()
        );
        if let Some(version) = get_user_version_for_file(&new_path) {
            return Some(version);
        }
    }

    None
}

pub fn get_user_version_for_file(path: impl AsRef<Path>) -> Option<UserVersion> {
    let file = std::fs::File::open(path).ok()?;
    let version = {
        let mut reader = DecodeReaderBytes::new(file);
        let mut version = String::new();
        reader.read_to_string(&mut version).map(|_| version)
    };

    match version {
        Err(err) => {
            info!("Can't read file: {}", err);
            None
        }
        Ok(version) => {
            info!("Found string {:?}  in version file", version);
            UserVersion::from_str(version.trim()).ok()
        }
    }
}
