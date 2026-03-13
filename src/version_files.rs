use crate::config::FnmConfig;
use crate::default_version;
use crate::nvmrc::Nvmrc;
use crate::package_json::PackageJson;
use crate::user_version::UserVersion;
use crate::version_file_format::VersionFileFormat;
use crate::version_file_strategy::VersionFileStrategy;
use encoding_rs_io::DecodeReaderBytes;
use log::info;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

const PATH_PARTS: [&str; 3] = [".nvmrc", ".node-version", "package.json"];

pub fn get_user_version_for_directory(
    path: impl AsRef<Path>,
    config: &FnmConfig,
) -> Option<UserVersion> {
    match config.version_file_strategy() {
        VersionFileStrategy::Local => get_user_version_for_single_directory(path, config),
        VersionFileStrategy::Recursive => get_user_version_for_directory_recursive(path, config)
            .or_else(|| {
                info!("Did not find anything recursively. Falling back to default alias.");
                default_version::find_default_version(config).map(UserVersion::Full)
            }),
    }
}

fn get_user_version_for_directory_recursive(
    path: impl AsRef<Path>,
    config: &FnmConfig,
) -> Option<UserVersion> {
    let mut current_path = Some(path.as_ref());

    while let Some(child_path) = current_path {
        if let Some(version) = get_user_version_for_single_directory(child_path, config) {
            return Some(version);
        }

        current_path = child_path.parent();
    }

    None
}

fn get_user_version_for_single_directory(
    path: impl AsRef<Path>,
    config: &FnmConfig,
) -> Option<UserVersion> {
    let path = path.as_ref();

    for path_part in &PATH_PARTS {
        let new_path = path.join(path_part);
        info!(
            "Looking for version file in {}. exists? {}",
            new_path.display(),
            new_path.exists()
        );
        if let Some(version) = get_user_version_for_file(&new_path, config) {
            return Some(version);
        }
    }

    None
}

pub fn get_user_version_for_file(
    path: impl AsRef<Path>,
    config: &FnmConfig,
) -> Option<UserVersion> {
    let format = VersionFileFormat::infer_from_path(path.as_ref());
    let file = std::fs::File::open(path).ok()?;
    let file = {
        let mut reader = DecodeReaderBytes::new(file);
        let mut version = String::new();
        reader.read_to_string(&mut version).map(|_| version)
    };

    match (file, format) {
        (Err(err), _) => {
            info!("Can't read file: {}", err);
            None
        }
        (Ok(pkg_json), VersionFileFormat::PackageJson) => {
            if !config.resolve_engines() {
                return None;
            }

            let pkg_json = serde_json::from_str::<PackageJson>(&pkg_json).ok();
            let range: Option<node_semver::Range> =
                pkg_json.as_ref().and_then(PackageJson::node_range).cloned();

            if let Some(range) = range {
                info!("Found package.json with {:?} in engines.node field", range);
                Some(UserVersion::SemverRange(range))
            } else {
                info!("No engines.node range found in package.json");
                None
            }
        }
        (Ok(nvmrc), VersionFileFormat::Nvmrc) => {
            let version = Nvmrc::parse(nvmrc).and_then(Nvmrc::version);
            match &version {
                Some(v) => info!("Parsed {:?} from .nvmrc", v),
                None => info!("Could not parse .nvmrc"),
            }
            version
        }
        (Ok(content), VersionFileFormat::Other) => {
            info!("Found string {:?} in version file", content);
            UserVersion::from_str(content.trim()).ok()
        }
    }
}
