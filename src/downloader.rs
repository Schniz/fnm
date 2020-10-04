use crate::archive;
use crate::archive::{Error as ExtractError, Extract};
use crate::directory_portal::DirectoryPortal;
use crate::version::Version;
use log::debug;
use reqwest::Url;
use snafu::{ensure, OptionExt, ResultExt, Snafu};
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum Error {
    HttpError {
        source: reqwest::Error,
    },
    IoError {
        source: std::io::Error,
    },
    #[snafu(display("Can't extract the file: {}", source))]
    CantExtractFile {
        source: ExtractError,
    },
    #[snafu(display("The downloaded archive is empty"))]
    TarIsEmpty,
    #[snafu(display("Can't find version upstream"))]
    VersionNotFound,
    #[snafu(display("Version already installed at {:?}", path))]
    VersionAlreadyInstalled {
        path: PathBuf,
    },
}

#[cfg(unix)]
fn filename_for_version(version: &Version) -> String {
    use crate::system_info::{platform_arch, platform_name};
    format!(
        "node-{node_ver}-{platform}-{arch}.tar.xz",
        node_ver = &version,
        platform = platform_name(),
        arch = platform_arch(),
    )
}

#[cfg(windows)]
fn filename_for_version(version: &Version) -> String {
    format!(
        "node-{node_ver}-win-{arch}.zip",
        node_ver = &version,
        arch = crate::system_info::platform_arch(),
    )
}

fn download_url(base_url: &Url, version: &Version) -> Url {
    Url::parse(&format!(
        "{}/{}/{}",
        base_url.as_str().trim_end_matches('/'),
        version,
        filename_for_version(version)
    ))
    .unwrap()
}

pub fn extract_archive_into<P: AsRef<Path>>(
    path: P,
    response: reqwest::blocking::Response,
) -> Result<(), Error> {
    #[cfg(unix)]
    let extractor = archive::TarXz::new(response);
    #[cfg(windows)]
    let extractor = archive::Zip::new(response);
    extractor.extract_into(path).context(CantExtractFile)?;
    Ok(())
}

/// Install a Node package
pub fn install_node_dist<P: AsRef<Path>>(
    version: &Version,
    node_dist_mirror: &Url,
    installations_dir: P,
) -> Result<(), Error> {
    let installation_dir = PathBuf::from(installations_dir.as_ref()).join(version.v_str());

    ensure!(
        !installation_dir.exists(),
        VersionAlreadyInstalled {
            path: PathBuf::from(installation_dir)
        }
    );

    std::fs::create_dir_all(installations_dir.as_ref()).context(IoError)?;
    let portal = DirectoryPortal::new(installation_dir);

    let url = download_url(node_dist_mirror, version);
    debug!("Going to call for {}", &url);
    let response = reqwest::blocking::get(url).context(HttpError)?;

    if response.status() == 404 {
        return Err(Error::VersionNotFound);
    }

    debug!("Extracting response...");
    extract_archive_into(&portal, response)?;
    debug!("Extraction completed");

    let installed_directory = std::fs::read_dir(&portal)
        .context(IoError)?
        .next()
        .context(TarIsEmpty)?
        .context(IoError)?;
    let installed_directory = installed_directory.path();

    let renamed_installation_dir = portal.join("installation");
    std::fs::rename(installed_directory, renamed_installation_dir).context(IoError)?;

    portal.teleport().context(IoError)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::downloader::install_node_dist;
    use crate::version::Version;
    use pretty_assertions::assert_eq;
    use std::io::Read;
    use tempfile::tempdir;

    #[test_env_log::test]
    fn test_installing_node_12() {
        let version = Version::parse("12.0.0").unwrap();
        let node_dist_mirror = Url::parse("https://nodejs.org/dist/").unwrap();
        let installations_dir = tempdir().unwrap();
        install_node_dist(&version, &node_dist_mirror, &installations_dir)
            .expect("Can't install Node 12");

        let mut location_path = PathBuf::from(&installations_dir.path());
        location_path.push(version.v_str());
        location_path.push("installation");

        if cfg!(unix) {
            location_path.push("bin");
        }

        location_path.push("node");

        let mut result = String::new();
        std::process::Command::new(location_path.to_str().unwrap())
            .arg("--version")
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Can't find node executable")
            .stdout
            .expect("Can't capture stdout")
            .read_to_string(&mut result)
            .expect("Failed reading stdout");
        assert_eq!(result.trim(), "v12.0.0");
    }
}
