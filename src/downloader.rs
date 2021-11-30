use crate::arch::Arch;
use crate::archive;
use crate::archive::{Error as ExtractError, Extract};
use crate::directory_portal::DirectoryPortal;
use crate::version::Version;
use log::debug;
use snafu::{ensure, OptionExt, ResultExt, Snafu};
use std::path::Path;
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Snafu)]
pub enum Error {
    HttpError {
        source: crate::http::Error,
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
    #[snafu(display(
        "{} for {} not found upstream.\nYou can `fnm ls-remote` to see available versions or try a different `--arch`.",
        version,
        arch
    ))]
    VersionNotFound {
        version: Version,
        arch: Arch,
    },
    #[snafu(display("Version already installed at {:?}", path))]
    VersionAlreadyInstalled {
        path: PathBuf,
    },
}

#[cfg(unix)]
fn filename_for_version(version: &Version, arch: &Arch) -> String {
    format!(
        "node-{node_ver}-{platform}-{arch}.tar.xz",
        node_ver = &version,
        platform = crate::system_info::platform_name(),
        arch = arch,
    )
}

#[cfg(windows)]
fn filename_for_version(version: &Version, arch: &Arch) -> String {
    format!(
        "node-{node_ver}-win-{arch}.zip",
        node_ver = &version,
        arch = arch,
    )
}

fn download_url(base_url: &Url, version: &Version, arch: &Arch) -> Url {
    Url::parse(&format!(
        "{}/{}/{}",
        base_url.as_str().trim_end_matches('/'),
        version,
        filename_for_version(version, arch)
    ))
    .unwrap()
}

pub fn extract_archive_into<P: AsRef<Path>>(
    path: P,
    response: crate::http::Response,
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
    arch: &Arch,
) -> Result<(), Error> {
    let installation_dir = PathBuf::from(installations_dir.as_ref()).join(version.v_str());

    ensure!(
        !installation_dir.exists(),
        VersionAlreadyInstalled {
            path: installation_dir
        }
    );

    std::fs::create_dir_all(installations_dir.as_ref()).context(IoError)?;

    let temp_installations_dir = installations_dir.as_ref().join(".downloads");
    std::fs::create_dir_all(&temp_installations_dir).context(IoError)?;

    let portal = DirectoryPortal::new_in(&temp_installations_dir, installation_dir);

    let url = download_url(node_dist_mirror, version, arch);
    debug!("Going to call for {}", &url);
    let response = crate::http::get(url.as_str()).context(HttpError)?;

    if response.status() == 404 {
        return Err(Error::VersionNotFound {
            version: version.clone(),
            arch: arch.clone(),
        });
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
    use tempfile::tempdir;

    #[test_env_log::test]
    fn test_installing_node_12() {
        let installations_dir = tempdir().unwrap();
        let node_path = install_in(installations_dir.path()).join("node");

        let stdout = duct::cmd(node_path.to_str().unwrap(), vec!["--version"])
            .stdout_capture()
            .run()
            .expect("Can't run Node binary")
            .stdout;

        let result = String::from_utf8(stdout).expect("Can't read `node --version` output");

        assert_eq!(result.trim(), "v12.0.0");
    }

    #[test_env_log::test]
    fn test_installing_npm() {
        let installations_dir = tempdir().unwrap();
        let npm_path = install_in(installations_dir.path()).join(if cfg!(windows) {
            "npm.cmd"
        } else {
            "npm"
        });

        let stdout = duct::cmd(npm_path.to_str().unwrap(), vec!["--version"])
            .stdout_capture()
            .run()
            .expect("Can't run npm")
            .stdout;

        let result = String::from_utf8(stdout).expect("Can't read npm output");

        assert_eq!(result.trim(), "6.9.0");
    }

    fn install_in(path: &Path) -> PathBuf {
        let version = Version::parse("12.0.0").unwrap();
        let arch = Arch::X64;
        let node_dist_mirror = Url::parse("https://nodejs.org/dist/").unwrap();
        install_node_dist(&version, &node_dist_mirror, &path, &arch)
            .expect("Can't install Node 12");

        let mut location_path = path.join(version.v_str()).join("installation");

        if cfg!(unix) {
            location_path.push("bin");
        }

        location_path
    }
}
