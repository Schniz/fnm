use crate::arch::Arch;
use crate::archive::{Archive, Error as ExtractError};
use crate::directory_portal::DirectoryPortal;
use crate::progress::ResponseProgress;
use crate::version::Version;
use indicatif::ProgressDrawTarget;
use log::debug;
use sha2::{Digest, Sha256};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    HttpError {
        #[from]
        source: crate::http::Error,
    },
    #[error(transparent)]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error("Can't extract the file: {}", source)]
    CantExtractFile {
        #[from]
        source: ExtractError,
    },
    #[error("The downloaded archive is empty")]
    TarIsEmpty,
    #[error("{} for {} not found upstream.\nYou can `fnm ls-remote` to see available versions or try a different `--arch`.", version, arch)]
    VersionNotFound { version: Version, arch: Arch },
    #[error("Version already installed at {:?}", path)]
    VersionAlreadyInstalled { path: PathBuf },
    #[error("Can't verify {file}: {url} has no checksum for it, so it wasn't installed")]
    ChecksumMissing { file: String, url: Url },
    #[error("{file} doesn't match its published checksum, so it wasn't installed.\nExpected SHA-256 {expected}, got {actual}.")]
    ChecksumMismatch {
        file: String,
        expected: String,
        actual: String,
    },
}

/// The mirror's list of checksums for `version`'s files. Node.js publishes
/// one per release, and mirrors copy it with the release.
fn checksums_url(base_url: &Url, version: &Version) -> Url {
    Url::parse(&format!(
        "{}/{}/SHASUMS256.txt",
        base_url.as_str().trim_end_matches('/'),
        version
    ))
    .unwrap()
}

/// The published SHA-256 of `file_name`, from the release's `SHASUMS256.txt`
/// at `url`. An error when the list can't be fetched or doesn't name the
/// file: with nothing to check against, the download isn't trusted.
fn published_checksum(url: &Url, file_name: &str) -> Result<String, Error> {
    let missing = || Error::ChecksumMissing {
        file: file_name.to_string(),
        url: url.clone(),
    };
    let response = crate::http::get(url.as_str())?;
    if !response.status().is_success() {
        return Err(missing());
    }
    let sums = response.text().map_err(crate::http::Error::from)?;
    checksum_for(&sums, file_name).ok_or_else(missing)
}

/// The checksum `sums` (the `sha256sum` format: a hex digest, white space,
/// a file name) gives `file_name`, lowercased.
fn checksum_for(sums: &str, file_name: &str) -> Option<String> {
    sums.lines().find_map(|line| {
        let mut fields = line.split_whitespace();
        let digest = fields.next()?;
        let name = fields.next()?.trim_start_matches('*');
        (name == file_name && digest.len() == 64 && digest.chars().all(|c| c.is_ascii_hexdigit()))
            .then(|| digest.to_ascii_lowercase())
    })
}

/// Reads the whole download into a temporary file while hashing it, and
/// hands the file back, rewound, only when the hash is `expected`. Nothing
/// is unpacked from a download that hasn't been checked.
fn download_verified(
    mut download: impl Read,
    file_name: &str,
    expected: &str,
) -> Result<std::fs::File, Error> {
    let mut file = tempfile::tempfile()?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0; 64 * 1024];
    loop {
        let read = download.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
        file.write_all(&buffer[..read])?;
    }
    let actual = format!("{:x}", hasher.finalize());
    if !actual.eq_ignore_ascii_case(expected) {
        return Err(Error::ChecksumMismatch {
            file: file_name.to_string(),
            expected: expected.to_string(),
            actual,
        });
    }
    file.seek(SeekFrom::Start(0))?;
    Ok(file)
}

#[cfg(unix)]
fn filename_for_version(version: &Version, arch: Arch, ext: &str) -> String {
    format!(
        "node-{node_ver}-{platform}-{arch}.{ext}",
        node_ver = version,
        platform = crate::system_info::platform_name(),
        arch = arch,
        ext = ext
    )
}

#[cfg(windows)]
fn filename_for_version(version: &Version, arch: Arch, ext: &str) -> String {
    format!(
        "node-{node_ver}-win-{arch}.{ext}",
        node_ver = &version,
        arch = arch,
        ext = ext,
    )
}

fn download_url(base_url: &Url, version: &Version, arch: Arch, ext: &str) -> Url {
    Url::parse(&format!(
        "{}/{}/{}",
        base_url.as_str().trim_end_matches('/'),
        version,
        filename_for_version(version, arch, ext)
    ))
    .unwrap()
}

/// Install a Node package
pub fn install_node_dist<P: AsRef<Path>>(
    version: &Version,
    node_dist_mirror: &Url,
    installations_dir: P,
    arch: Arch,
    show_progress: bool,
) -> Result<(), Error> {
    let installation_dir = PathBuf::from(installations_dir.as_ref()).join(version.v_str());

    if installation_dir.exists() {
        return Err(Error::VersionAlreadyInstalled {
            path: installation_dir,
        });
    }

    std::fs::create_dir_all(installations_dir.as_ref())?;

    let temp_installations_dir = installations_dir.as_ref().join(".downloads");
    std::fs::create_dir_all(&temp_installations_dir)?;

    let portal = DirectoryPortal::new_in(&temp_installations_dir, installation_dir);

    for extract in Archive::supported() {
        let ext = extract.file_extension();
        let url = download_url(node_dist_mirror, version, arch, ext);
        debug!("Going to call for {url}");
        let response = crate::http::get(url.as_str())?;

        if !response.status().is_success() {
            continue;
        }

        // What the mirror publishes for this file, before anything is
        // downloaded or unpacked: a file without a published checksum isn't
        // installed.
        let file_name = filename_for_version(version, arch, ext);
        let checksums_url = checksums_url(node_dist_mirror, version);
        let expected = published_checksum(&checksums_url, &file_name)?;

        debug!("Downloading and checking {file_name}...");
        let archive = if show_progress {
            download_verified(
                ResponseProgress::new(response, ProgressDrawTarget::stderr()),
                &file_name,
                &expected,
            )?
        } else {
            download_verified(response, &file_name, &expected)?
        };

        debug!("Extracting {file_name}...");
        extract.extract_archive_into(portal.as_ref(), archive)?;
        debug!("Extraction completed");

        let installed_directory = std::fs::read_dir(&portal)?
            .next()
            .ok_or(Error::TarIsEmpty)??;
        let installed_directory = installed_directory.path();

        let renamed_installation_dir = portal.join("installation");
        std::fs::rename(installed_directory, renamed_installation_dir)?;

        portal.teleport()?;

        return Ok(());
    }

    Err(Error::VersionNotFound {
        version: version.clone(),
        arch,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::downloader::install_node_dist;
    use crate::version::Version;
    use pretty_assertions::assert_eq;
    use tempfile::tempdir;

    #[test_log::test]
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

    #[test_log::test]
    fn test_installing_npm() {
        let installations_dir = tempdir().unwrap();
        let bin_dir = install_in(installations_dir.path());
        let npm_path = bin_dir.join(if cfg!(windows) { "npm.cmd" } else { "npm" });

        let stdout = duct::cmd(npm_path.to_str().unwrap(), vec!["--version"])
            .env("PATH", bin_dir)
            .stdout_capture()
            .run()
            .expect("Can't run npm")
            .stdout;

        let result = String::from_utf8(stdout).expect("Can't read npm output");

        assert_eq!(result.trim(), "6.9.0");
    }

    #[test]
    fn checksums_are_looked_up_by_exact_file_name() {
        let sums = "\
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa  node-v20.0.0-win-x64.zip
BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB *node-v20.0.0-linux-x64.tar.xz
cccc  node-v20.0.0-darwin-arm64.tar.gz
";
        assert_eq!(
            checksum_for(sums, "node-v20.0.0-win-x64.zip").as_deref(),
            Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
        );
        // Binary-mode marker and upper case are accepted; lowercased.
        assert_eq!(
            checksum_for(sums, "node-v20.0.0-linux-x64.tar.xz").as_deref(),
            Some("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
        );
        // A malformed digest is no checksum.
        assert_eq!(checksum_for(sums, "node-v20.0.0-darwin-arm64.tar.gz"), None);
        assert_eq!(checksum_for(sums, "node-v20.0.0-win-x64.zip.sig"), None);
        assert_eq!(checksum_for(sums, "node-v20.0.0-win-x64"), None);
    }

    #[test]
    fn only_a_download_that_matches_its_checksum_is_handed_on() {
        let content = b"pretend archive";
        // sha256 of "pretend archive"
        let digest = format!("{:x}", Sha256::digest(content));

        let mut file = download_verified(&content[..], "x.zip", &digest).unwrap();
        let mut read_back = Vec::new();
        file.read_to_end(&mut read_back).unwrap();
        assert_eq!(read_back, content, "rewound and complete");

        let wrong = "0".repeat(64);
        let error = download_verified(&content[..], "x.zip", &wrong).unwrap_err();
        assert!(matches!(error, Error::ChecksumMismatch { .. }), "{error}");
    }

    #[test]
    fn the_checksum_list_sits_beside_the_release() {
        let mirror = Url::parse("https://nodejs.org/dist/").unwrap();
        let version = Version::parse("20.0.0").unwrap();
        assert_eq!(
            checksums_url(&mirror, &version).as_str(),
            "https://nodejs.org/dist/v20.0.0/SHASUMS256.txt"
        );
    }

    fn install_in(path: &Path) -> PathBuf {
        let version = Version::parse("12.0.0").unwrap();
        let arch = Arch::X64;
        let node_dist_mirror = Url::parse("https://nodejs.org/dist/").unwrap();
        install_node_dist(&version, &node_dist_mirror, path, arch, false)
            .expect("Can't install Node 12");

        let mut location_path = path.join(version.v_str()).join("installation");

        if cfg!(unix) {
            location_path.push("bin");
        }

        location_path
    }
}
