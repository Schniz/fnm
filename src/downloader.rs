use crate::arch::Arch;
use crate::archive::{Archive, Error as ExtractError};
use crate::directory_portal::DirectoryPortal;
use crate::download_verification::{self, Check, DownloadVerification, WhenMissing};
use crate::log_level::LogLevel;
use crate::outln;
use crate::progress::ResponseProgress;
use crate::release_keys;
use crate::version::Version;
use colored::Colorize;
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
    #[error("Can't verify the download: {url} returned HTTP {status}, so nothing was installed.\n{when_missing}")]
    ChecksumsMissing {
        url: Url,
        status: u16,
        when_missing: WhenMissing,
    },
    #[error("Can't verify {url}, so nothing was installed: {source}")]
    SignatureNotVerified {
        url: Url,
        #[source]
        source: Box<release_keys::Error>,
    },
    #[error("Can't verify {file}: {url} has no checksum for it, so it wasn't installed")]
    ChecksumMissing { file: String, url: Url },
    #[error("{url} doesn't match its published checksum, so it wasn't installed.\nExpected SHA-256 {expected}, got {actual}.")]
    ChecksumMismatch {
        url: String,
        expected: String,
        actual: String,
    },
    #[error("{url} is larger than {limit} bytes, so nothing was installed")]
    TooLarge { url: Url, limit: u64 },
}

/// Far above what Node.js publishes, so a hostile mirror can't fill the disk.
const MAX_CHECKSUMS_SIZE: u64 = 1024 * 1024;
const MAX_ARCHIVE_SIZE: u64 = 512 * 1024 * 1024;

/// A file the mirror publishes beside `version`'s archives, such as its
/// list of checksums. Node.js publishes one per release, and mirrors copy it
/// with the release.
fn release_file_url(base_url: &Url, version: &Version, file_name: &str) -> Url {
    Url::parse(&format!(
        "{}/{}/{file_name}",
        base_url.as_str().trim_end_matches('/'),
        version
    ))
    .unwrap()
}

/// How `install_node_dist` checks what it downloads.
pub struct Verification {
    pub mode: DownloadVerification,
    /// Where release keys refreshed from GitHub are kept.
    pub release_keys_dir: PathBuf,
    /// More release keys to trust, from `--node-release-keys`.
    pub user_keys: Option<PathBuf>,
    pub log_level: LogLevel,
}

impl Verification {
    fn log_level(&self) -> LogLevel {
        self.log_level
    }
}

/// The release's `SHASUMS256.txt(.asc)` at `url`. An error when it can't be
/// fetched: with nothing to check against, no download is trusted.
fn fetch_checksums(url: &Url, when_missing: WhenMissing) -> Result<String, Error> {
    let response = crate::http::get(url.as_str())?;
    if !response.status().is_success() {
        return Err(Error::ChecksumsMissing {
            url: url.clone(),
            status: response.status().as_u16(),
            when_missing,
        });
    }
    let mut body = vec![];
    response
        .take(MAX_CHECKSUMS_SIZE + 1)
        .read_to_end(&mut body)?;
    if body.len() as u64 > MAX_CHECKSUMS_SIZE {
        return Err(Error::TooLarge {
            url: url.clone(),
            limit: MAX_CHECKSUMS_SIZE,
        });
    }
    Ok(String::from_utf8_lossy(&body).into_owned())
}

/// The checksums in the clearsigned `signed` of `version`, fetched from `url`:
/// only the signed text, never what the file has around it.
fn verified_checksums(
    url: &Url,
    signed: &str,
    version: &Version,
    verification: &Verification,
) -> Result<String, Error> {
    let verified = release_keys::verify(
        signed,
        &version.v_str(),
        &verification.release_keys_dir,
        verification.user_keys.as_deref(),
        &release_keys::fetch_from_github,
    )
    .map_err(|source| Error::SignatureNotVerified {
        url: url.clone(),
        source: Box::new(source),
    })?;
    debug!("Signed with key {}", verified.signing_key);
    let user_key = match &verified.user_keys {
        Some(path) => format!(", user key from {}", path.display()),
        None => String::new(),
    };
    outln!(
        verification,
        Info,
        "Verified signature by {} ({}{user_key})",
        verified.signer,
        verified.fingerprint
    );
    Ok(verified.signed_text)
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
/// is unpacked from a download that hasn't been checked, or that is larger
/// than `limit`.
fn download_verified(
    download: impl Read,
    url: &Url,
    expected: &str,
    limit: u64,
) -> Result<std::fs::File, Error> {
    let mut file = tempfile::tempfile()?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0; 64 * 1024];
    let mut download = download.take(limit + 1);
    let mut total = 0;
    loop {
        let read = download.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        total += read as u64;
        if total > limit {
            return Err(Error::TooLarge {
                url: url.clone(),
                limit,
            });
        }
        hasher.update(&buffer[..read]);
        file.write_all(&buffer[..read])?;
    }
    let actual = format!("{:x}", hasher.finalize());
    if !actual.eq_ignore_ascii_case(expected) {
        return Err(Error::ChecksumMismatch {
            url: url.to_string(),
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
    verification: &Verification,
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

    let policy = download_verification::policy(verification.mode, node_dist_mirror, version);
    if let Some(warning) = policy.warning() {
        outln!(
            verification,
            Error,
            "{} {}",
            "warning:".bold().yellow(),
            warning
        );
    }

    // One checksum list per release, fetched before any archive.
    let checksums = match policy.check {
        Check::None => None,
        Check::Checksum { .. } => {
            let url = release_file_url(node_dist_mirror, version, "SHASUMS256.txt");
            let checksums = fetch_checksums(&url, policy.when_missing)?;
            Some((url, checksums))
        }
        Check::Signature => {
            let url = release_file_url(node_dist_mirror, version, "SHASUMS256.txt.asc");
            let signed = fetch_checksums(&url, policy.when_missing)?;
            let checksums = verified_checksums(&url, &signed, version, verification)?;
            Some((url, checksums))
        }
    };

    for extract in Archive::supported() {
        let ext = extract.file_extension();
        let url = download_url(node_dist_mirror, version, arch, ext);
        debug!("Going to call for {url}");
        let response = crate::http::get(url.as_str())?;

        if !response.status().is_success() {
            continue;
        }

        let file_name = filename_for_version(version, arch, ext);
        let response: Box<dyn Read> = if show_progress {
            Box::new(ResponseProgress::new(
                response,
                ProgressDrawTarget::stderr(),
            ))
        } else {
            Box::new(response)
        };

        if let Some((checksums_url, checksums)) = &checksums {
            // A file without a published checksum isn't installed.
            let expected =
                checksum_for(checksums, &file_name).ok_or_else(|| Error::ChecksumMissing {
                    file: file_name.clone(),
                    url: checksums_url.clone(),
                })?;
            debug!("Downloading and checking {file_name}...");
            let archive = download_verified(response, &url, &expected, MAX_ARCHIVE_SIZE)?;
            debug!("Extracting {file_name}...");
            extract.extract_archive_into(portal.as_ref(), archive)?;
        } else {
            debug!("Extracting {file_name} unchecked...");
            extract.extract_archive_into(portal.as_ref(), response)?;
        }
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

        let url = Url::parse("https://example.com/x.zip").unwrap();
        let mut file = download_verified(&content[..], &url, &digest, 1024).unwrap();
        let mut read_back = Vec::new();
        file.read_to_end(&mut read_back).unwrap();
        assert_eq!(read_back, content, "rewound and complete");

        let wrong = "0".repeat(64);
        let error = download_verified(&content[..], &url, &wrong, 1024).unwrap_err();
        assert!(matches!(error, Error::ChecksumMismatch { .. }), "{error}");
        assert!(error.to_string().starts_with("https://example.com/x.zip "));

        let limit = content.len() as u64 - 1;
        let error = download_verified(&content[..], &url, &digest, limit).unwrap_err();
        assert!(matches!(error, Error::TooLarge { .. }), "{error}");
    }

    #[test]
    fn test_checksums_come_only_from_the_signed_text() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/release-signatures/v20.0.0/SHASUMS256.txt.asc"
        );
        // Armor headers aren't signed, so this one passes the signature check,
        // and reads as a checksum line.
        let forged = format!("{}  node-v20.0.0-forged.tar.xz : x", "0".repeat(64));
        let signed = std::fs::read_to_string(path).unwrap().replace(
            "-----BEGIN PGP SIGNATURE-----\n",
            &format!("-----BEGIN PGP SIGNATURE-----\n{forged}\n"),
        );
        assert!(checksum_for(&signed, "node-v20.0.0-forged.tar.xz").is_some());

        let url = Url::parse("https://nodejs.org/dist/v20.0.0/SHASUMS256.txt.asc").unwrap();
        let cache = tempfile::tempdir().unwrap();
        let verification = Verification {
            mode: DownloadVerification::Signature,
            release_keys_dir: cache.path().join("release-keys"),
            user_keys: None,
            log_level: LogLevel::Quiet,
        };
        let version = Version::parse("20.0.0").unwrap();
        let checksums = verified_checksums(&url, &signed, &version, &verification).unwrap();
        assert_eq!(checksum_for(&checksums, "node-v20.0.0-forged.tar.xz"), None);
        assert!(checksum_for(&checksums, "node-v20.0.0-linux-x64.tar.xz").is_some());
    }

    #[test]
    fn the_checksum_list_sits_beside_the_release() {
        let mirror = Url::parse("https://nodejs.org/dist/").unwrap();
        let version = Version::parse("20.0.0").unwrap();
        assert_eq!(
            release_file_url(&mirror, &version, "SHASUMS256.txt").as_str(),
            "https://nodejs.org/dist/v20.0.0/SHASUMS256.txt"
        );
    }

    fn install_in(path: &Path) -> PathBuf {
        let version = Version::parse("12.0.0").unwrap();
        let arch = Arch::X64;
        let node_dist_mirror = Url::parse("https://nodejs.org/dist/").unwrap();
        let verification = Verification {
            mode: DownloadVerification::default(),
            release_keys_dir: path.join("release-keys"),
            user_keys: None,
            log_level: LogLevel::Info,
        };
        install_node_dist(
            &version,
            &node_dist_mirror,
            path,
            arch,
            false,
            &verification,
        )
        .expect("Can't install Node 12");

        let mut location_path = path.join(version.v_str()).join("installation");

        if cfg!(unix) {
            location_path.push("bin");
        }

        location_path
    }
}
