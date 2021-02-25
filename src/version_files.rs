use crate::user_version::UserVersion;
use encoding_rs_io::DecodeReaderBytes;
use log::info;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

const PATH_PARTS: [&str; 2] = [".nvmrc", ".node-version"];

pub fn get_user_version_for_directory(path: impl AsRef<Path>) -> Option<UserVersion> {
    let path = path.as_ref();

    for path_part in PATH_PARTS.iter() {
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
