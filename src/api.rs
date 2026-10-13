//! The library interface to fnm: the same operations as the commands, as
//! functions that return their result.
//!
//! Everything here takes plain strings and paths and gives plain data back,
//! so nothing of fnm's internals has to be named by the caller. A version is
//! asked for the way it is on the command line: `22`, `22.3`, `v22.3.0`,
//! `lts`, `lts/iron`, `latest`, or an alias such as `default`.
//!
//! Every call that touches the network or the disk blocks until it is done.
//! Call them from a thread that is allowed to block, not from an async task.

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::Parser as _;

use crate::alias::{create_alias, get_alias_by_name, list_aliases, StoredAlias};
use crate::arch::get_safe_arch;
use crate::choose_version_for_user_input::{
    choose_version_for_user_input, Error as ChooseVersionError,
};
use crate::config::FnmConfig;
use crate::downloader::{install_node_dist, Error as DownloadError};
use crate::fs::remove_symlink_dir;
use crate::lts::LtsType;
use crate::user_version::UserVersion;
use crate::version::Version;
use crate::version_file_strategy::VersionFileStrategy;
use crate::version_files::get_user_version_for_directory;
use crate::{installed_versions, remote_node_index};

/// A Node.js version that is installed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledVersion {
    /// The exact version, with its leading `v`: `v22.3.0`.
    pub version: String,
    /// The directory the Node.js distribution was unpacked into.
    pub path: PathBuf,
    /// The directory holding `node`, `npm` and `npx`. Putting it first on
    /// `PATH` is all it takes to run a command under this version.
    pub bin_dir: PathBuf,
    /// The aliases pointing at this version, sorted.
    pub aliases: Vec<String>,
    /// Whether this is the default version.
    pub is_default: bool,
}

/// A Node.js version that can be downloaded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteVersion {
    /// The exact version, with its leading `v`: `v22.3.0`.
    pub version: String,
    /// The release line's codename when this is a long-term-support
    /// release, such as `Jod`.
    pub lts: Option<String>,
}

/// What can go wrong in a call to [`Fnm`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The text isn't a version, a version range, or an alias.
    #[error("\"{0}\" isn't a Node.js version fnm understands")]
    InvalidVersion(String),
    /// Nothing installed matches what was asked for.
    #[error("Node.js {0} isn't installed")]
    NotInstalled(String),
    /// More than one installed version matches, where exactly one is needed.
    #[error("\"{requested}\" matches more than one installed version: {}", matches.join(", "))]
    Ambiguous {
        /// What was asked for.
        requested: String,
        /// The installed versions it matches.
        matches: Vec<String>,
    },
    /// The download index has nothing matching what was asked for.
    #[error("no Node.js version matching \"{0}\" is available to download")]
    NotAvailable(String),
    /// The request names an alias or the system Node.js, not a release.
    #[error("\"{0}\" isn't a version that can be installed")]
    NotInstallable(String),
    /// The installed versions couldn't be read from disk.
    #[error("can't list the installed Node.js versions: {0}")]
    ListInstalled(String),
    /// The download index couldn't be fetched or read.
    #[error("can't list the Node.js versions available to download: {0}")]
    ListAvailable(String),
    /// A download or its unpacking failed.
    #[error("can't install Node.js {version}: {message}")]
    Install {
        /// The version being installed.
        version: String,
        /// Why it failed.
        message: String,
    },
    /// The download mirror isn't a URL.
    #[error("\"{0}\" isn't a valid URL for a Node.js download mirror")]
    InvalidMirror(String),
    /// A file or directory operation failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// A handle on one fnm installation: where its Node.js versions are kept and
/// where new ones are downloaded from.
#[derive(Debug, Clone)]
pub struct Fnm {
    config: FnmConfig,
}

impl Default for Fnm {
    fn default() -> Self {
        Self::new()
    }
}

impl Fnm {
    /// The installation the `fnm` program itself would use: its directory,
    /// mirror and architecture come from the same `FNM_*` environment
    /// variables, with the same defaults.
    ///
    /// Versions installed through this are the ones `fnm` on the command
    /// line sees, and the other way round.
    #[must_use]
    pub fn new() -> Self {
        // `FnmConfig` is the command line's own settings type, so parsing an
        // empty command line is what applies the environment and defaults.
        let config = FnmConfig::try_parse_from(std::iter::once("fnm"))
            .unwrap_or_else(|_| FnmConfig::default());
        Self { config }
    }

    /// An installation kept in `base_dir`, ignoring the environment.
    ///
    /// This is how an application keeps Node.js versions of its own, apart
    /// from any fnm the user has.
    #[must_use]
    pub fn in_dir(base_dir: impl Into<PathBuf>) -> Self {
        let mut config = FnmConfig::default();
        config.base_dir = Some(base_dir.into());
        Self { config }
    }

    /// Downloads from `mirror` instead of `https://nodejs.org/dist`.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidMirror`] when `mirror` isn't a URL.
    pub fn with_mirror(mut self, mirror: &str) -> Result<Self, Error> {
        self.config.node_dist_mirror =
            url::Url::parse(mirror).map_err(|_| Error::InvalidMirror(mirror.to_string()))?;
        Ok(self)
    }

    /// Whether [`Self::requested_version`] also looks in the directories
    /// above the one it is given. Off by default, as it is for `fnm`.
    #[must_use]
    pub fn search_parent_directories(mut self, search: bool) -> Self {
        let strategy = if search {
            VersionFileStrategy::Recursive
        } else {
            VersionFileStrategy::Local
        };
        self.config = self.config.with_version_file_strategy(strategy);
        self
    }

    /// The directory this installation keeps everything in.
    #[must_use]
    pub fn base_dir(&self) -> PathBuf {
        self.config.base_dir_with_default()
    }

    /// Every installed version, oldest first.
    ///
    /// # Errors
    ///
    /// [`Error::ListInstalled`] when the installation directory can't be
    /// read.
    pub fn installed(&self) -> Result<Vec<InstalledVersion>, Error> {
        let mut versions = self.installed_versions()?;
        versions.sort();
        let aliases = list_aliases(&self.config).unwrap_or_default();
        Ok(versions
            .iter()
            .map(|version| self.describe(version, &aliases))
            .collect())
    }

    /// Every version that can be downloaded, oldest first.
    ///
    /// # Errors
    ///
    /// [`Error::ListAvailable`] when the index can't be fetched from the
    /// mirror or isn't what was expected.
    pub fn available(&self) -> Result<Vec<RemoteVersion>, Error> {
        let versions = self.remote_versions()?;
        Ok(versions
            .into_iter()
            .map(|indexed| RemoteVersion {
                version: indexed.version.v_str(),
                lts: indexed.lts,
            })
            .collect())
    }

    /// The version `directory` asks for, read from its `.nvmrc`,
    /// `.node-version`, or the `engines.node` field of its `package.json`.
    ///
    /// This is the request as written, such as `22` or `>=20`, not an
    /// installed version: pass it to [`Self::resolve`] or [`Self::install`].
    /// `None` when the directory doesn't ask for one.
    #[must_use]
    pub fn requested_version(&self, directory: impl AsRef<Path>) -> Option<String> {
        get_user_version_for_directory(directory, &self.config)
            .map(|requested| requested.to_string())
    }

    /// The installed version that `request` selects: the newest one that
    /// matches it.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidVersion`] when `request` can't be read as a version,
    /// [`Error::NotInstalled`] when nothing installed matches it, and
    /// [`Error::ListInstalled`] when the installed versions can't be read.
    pub fn resolve(&self, request: &str) -> Result<InstalledVersion, Error> {
        let requested = parse_request(request)?;
        let version = self.installed_version_for(request, &requested)?;
        let aliases = list_aliases(&self.config).unwrap_or_default();
        Ok(self.describe(&version, &aliases))
    }

    /// The installed version that `directory` selects through its `.nvmrc`,
    /// `.node-version` or `package.json`. `None` when it asks for nothing.
    ///
    /// Unlike passing [`Self::requested_version`] to [`Self::resolve`], this
    /// also works for a `package.json` version range such as `>=20`, which
    /// can't be written as a single version to ask for.
    ///
    /// # Errors
    ///
    /// [`Error::NotInstalled`] when nothing installed satisfies what the
    /// directory asks for, and [`Error::ListInstalled`] when the installed
    /// versions can't be read.
    pub fn resolve_for(
        &self,
        directory: impl AsRef<Path>,
    ) -> Result<Option<InstalledVersion>, Error> {
        let Some(requested) = get_user_version_for_directory(directory, &self.config) else {
            return Ok(None);
        };
        let version = self.installed_version_for(&requested.to_string(), &requested)?;
        let aliases = list_aliases(&self.config).unwrap_or_default();
        Ok(Some(self.describe(&version, &aliases)))
    }

    /// Downloads and unpacks the newest version satisfying what `directory`
    /// asks for, as [`Self::install`] does for a version given by name.
    /// `None` when the directory asks for nothing.
    ///
    /// # Errors
    ///
    /// The same as [`Self::install`].
    pub fn install_for(
        &self,
        directory: impl AsRef<Path>,
    ) -> Result<Option<InstalledVersion>, Error> {
        let Some(requested) = get_user_version_for_directory(directory, &self.config) else {
            return Ok(None);
        };
        self.install_requested(&requested.to_string(), &requested)
            .map(Some)
    }

    /// Downloads and unpacks the newest version matching `request`, and
    /// returns it. A version that is already installed is returned as it is.
    ///
    /// The first version ever installed becomes the default, and installing
    /// `lts` or `latest` points the alias of that name at the result, both
    /// as `fnm install` does. Corepack is not enabled.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidVersion`] or [`Error::NotInstallable`] when `request`
    /// isn't a release that can be downloaded, [`Error::ListAvailable`] and
    /// [`Error::NotAvailable`] when it can't be matched against the mirror,
    /// and [`Error::Install`] when the download or unpacking fails.
    pub fn install(&self, request: &str) -> Result<InstalledVersion, Error> {
        let requested = parse_install_request(request)?;
        self.install_requested(request, &requested)
    }

    fn install_requested(
        &self,
        request: &str,
        requested: &UserVersion,
    ) -> Result<InstalledVersion, Error> {
        let version = self.downloadable_version_for(request, requested)?;

        let arch = get_safe_arch(self.config.arch, &version);
        match install_node_dist(
            &version,
            &self.config.node_dist_mirror,
            self.config.installations_dir(),
            arch,
            false,
        ) {
            Ok(()) | Err(DownloadError::VersionAlreadyInstalled { .. }) => {}
            Err(error) => {
                return Err(Error::Install {
                    version: version.v_str(),
                    message: error.to_string(),
                });
            }
        }

        if !self.config.default_version_dir().exists() {
            create_alias(&self.config, "default", &version)?;
        }
        if let Some(alias) = requested.inferred_alias() {
            create_alias(&self.config, &alias.v_str(), &version)?;
        }

        let aliases = list_aliases(&self.config).unwrap_or_default();
        Ok(self.describe(&version, &aliases))
    }

    /// Removes the one installed version matching `request`, along with the
    /// aliases pointing at it, and returns which version that was.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidVersion`] when `request` can't be read as a version,
    /// [`Error::NotInstalled`] when nothing installed matches it,
    /// [`Error::Ambiguous`] when more than one does, and [`Error::Io`] when
    /// the files can't be removed.
    pub fn uninstall(&self, request: &str) -> Result<String, Error> {
        let requested = parse_request(request)?;
        if matches!(requested, UserVersion::Full(Version::Bypassed)) {
            return Err(Error::NotInstalled(request.to_string()));
        }

        let mut installed = self.installed_versions()?;
        installed.sort();
        let matches: Vec<String> = installed
            .iter()
            .filter(|version| requested.matches(version, &self.config))
            .map(Version::v_str)
            .collect();
        if matches.len() > 1 {
            return Err(Error::Ambiguous {
                requested: request.to_string(),
                matches,
            });
        }
        let version = requested
            .to_version(&installed, &self.config)
            .ok_or_else(|| Error::NotInstalled(request.to_string()))?;

        let aliases = version.find_aliases(&self.config)?;
        let root = version
            .root_path(&self.config)
            .ok_or_else(|| Error::NotInstalled(request.to_string()))?;
        std::fs::remove_dir_all(root)?;
        for alias in aliases {
            remove_symlink_dir(alias.path())?;
        }
        Ok(version.v_str())
    }

    /// Makes the installed version matching `request` the default, and
    /// returns it.
    ///
    /// # Errors
    ///
    /// The same as [`Self::resolve`], and [`Error::Io`] when the alias can't
    /// be written.
    pub fn set_default(&self, request: &str) -> Result<InstalledVersion, Error> {
        let requested = parse_request(request)?;
        let version = self.installed_version_for(request, &requested)?;
        create_alias(&self.config, "default", &version)?;
        let aliases = list_aliases(&self.config).unwrap_or_default();
        Ok(self.describe(&version, &aliases))
    }

    /// The default version, or `None` when there isn't one yet.
    #[must_use]
    pub fn default_version(&self) -> Option<String> {
        get_alias_by_name(&self.config, "default").map(|alias| alias.s_ver().to_string())
    }

    /// The value to give a child process as `PATH` so that it runs under the
    /// installed version matching `request`: that version's
    /// [`bin_dir`](InstalledVersion::bin_dir), followed by this process's
    /// own `PATH`.
    ///
    /// This is what `fnm exec` does, without running anything.
    ///
    /// # Errors
    ///
    /// The same as [`Self::resolve`], and [`Error::Io`] when the directory's
    /// name can't be part of a `PATH`.
    pub fn path_for(&self, request: &str) -> Result<OsString, Error> {
        let version = self.resolve(request)?;
        let current = std::env::var_os("PATH").unwrap_or_default();
        let mut paths = vec![version.bin_dir];
        paths.extend(std::env::split_paths(&current));
        std::env::join_paths(paths)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidInput, error).into())
    }

    fn installed_versions(&self) -> Result<Vec<Version>, Error> {
        installed_versions::list(self.config.installations_dir())
            .map_err(|error| Error::ListInstalled(error.to_string()))
    }

    fn remote_versions(&self) -> Result<Vec<remote_node_index::IndexedNodeVersion>, Error> {
        remote_node_index::list(&self.config.node_dist_mirror)
            .map_err(|error| Error::ListAvailable(error.to_string()))
    }

    /// The exact installed version `requested` selects. An alias is followed
    /// to the version it points at.
    fn installed_version_for(
        &self,
        request: &str,
        requested: &UserVersion,
    ) -> Result<Version, Error> {
        let not_installed = || Error::NotInstalled(request.to_string());
        let applicable = match choose_version_for_user_input(requested, &self.config) {
            Ok(applicable) => applicable.ok_or_else(not_installed)?,
            Err(ChooseVersionError::CantFindVersion { .. }) => return Err(not_installed()),
            Err(ChooseVersionError::VersionListing { source }) => {
                return Err(Error::ListInstalled(source.to_string()));
            }
        };
        match applicable.version() {
            version @ Version::Semver(_) => Ok(version.clone()),
            Version::Alias(name) => get_alias_by_name(&self.config, name)
                .and_then(|alias| Version::parse(alias.s_ver()).ok())
                .filter(|version| matches!(version, Version::Semver(_)))
                .ok_or_else(not_installed),
            // The system Node.js isn't one of fnm's installations.
            Version::Bypassed | Version::Lts(_) | Version::Latest => Err(not_installed()),
        }
    }

    /// The exact release `requested` selects from the download index.
    ///
    /// This follows `fnm install`, which works the same answer out inline
    /// while reporting its progress.
    fn downloadable_version_for(
        &self,
        request: &str,
        requested: &UserVersion,
    ) -> Result<Version, Error> {
        let not_available = || Error::NotAvailable(request.to_string());
        match requested {
            UserVersion::Full(version @ Version::Semver(_)) => Ok(version.clone()),
            UserVersion::Full(Version::Bypassed | Version::Alias(_)) => {
                Err(Error::NotInstallable(request.to_string()))
            }
            UserVersion::Full(Version::Lts(lts)) => {
                let available = self.remote_versions()?;
                lts.pick_latest(&available)
                    .map(|indexed| indexed.version.clone())
                    .ok_or_else(not_available)
            }
            UserVersion::Full(Version::Latest) => self
                .remote_versions()?
                .last()
                .map(|indexed| indexed.version.clone())
                .ok_or_else(not_available),
            partial => {
                let available: Vec<Version> = self
                    .remote_versions()?
                    .into_iter()
                    .map(|indexed| indexed.version)
                    .collect();
                partial
                    .to_version(&available, &self.config)
                    .cloned()
                    .ok_or_else(not_available)
            }
        }
    }

    fn describe(&self, version: &Version, aliases: &[StoredAlias]) -> InstalledVersion {
        let name = version.v_str();
        let path = version.installation_path(&self.config);
        let mut pointing_here: Vec<String> = aliases
            .iter()
            .filter(|alias| alias.s_ver() == name)
            .map(|alias| alias.name().to_string())
            .collect();
        pointing_here.sort();
        InstalledVersion {
            is_default: pointing_here.iter().any(|alias| alias == "default"),
            bin_dir: bin_dir(&path),
            aliases: pointing_here,
            version: name,
            path,
        }
    }
}

/// Where a Node.js distribution keeps its executables: beside everything
/// else on Windows, in `bin` elsewhere.
fn bin_dir(installation: &Path) -> PathBuf {
    if cfg!(windows) {
        installation.to_path_buf()
    } else {
        installation.join("bin")
    }
}

/// Reads what to install. On top of [`parse_request`], `latest` means the
/// newest release there is, which on the command line is the `--latest`
/// flag.
///
/// Only installing reads it that way. Everywhere else `latest` is the alias
/// of that name, which installing `latest` leaves pointing at the result.
fn parse_install_request(request: &str) -> Result<UserVersion, Error> {
    if request.trim().eq_ignore_ascii_case("latest") {
        return Ok(UserVersion::Full(Version::Latest));
    }
    parse_request(request)
}

/// Reads a version the way the command line does, with one addition: a bare
/// `lts` means the newest long-term-support release, which on the command
/// line is the `--lts` flag.
fn parse_request(request: &str) -> Result<UserVersion, Error> {
    let trimmed = request.trim();
    if trimmed.is_empty() {
        return Err(Error::InvalidVersion(request.to_string()));
    }
    if trimmed.eq_ignore_ascii_case("lts") {
        return Ok(UserVersion::Full(Version::Lts(LtsType::Latest)));
    }
    UserVersion::from_str(trimmed).map_err(|_| Error::InvalidVersion(request.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    /// An installation in a temporary directory holding `versions`, each as
    /// the empty directory fnm would have unpacked a distribution into.
    fn installation(versions: &[&str]) -> (tempfile::TempDir, Fnm) {
        let base_dir = tempfile::tempdir().unwrap();
        for version in versions {
            std::fs::create_dir_all(
                base_dir
                    .path()
                    .join("node-versions")
                    .join(version)
                    .join("installation"),
            )
            .unwrap();
        }
        let fnm = Fnm::in_dir(base_dir.path());
        (base_dir, fnm)
    }

    fn names(versions: &[InstalledVersion]) -> Vec<&str> {
        versions
            .iter()
            .map(|version| version.version.as_str())
            .collect()
    }

    #[test]
    fn lists_installed_versions_oldest_first() {
        let (_dir, fnm) = installation(&["v22.3.0", "v18.20.4", "v20.11.1"]);
        let installed = fnm.installed().unwrap();
        assert_eq!(names(&installed), vec!["v18.20.4", "v20.11.1", "v22.3.0"]);

        let newest = installed.last().unwrap();
        assert!(newest.path.ends_with("v22.3.0/installation"));
        assert_eq!(newest.bin_dir, bin_dir(&newest.path));
        assert!(newest.aliases.is_empty());
        assert!(!newest.is_default);
    }

    #[test]
    fn an_empty_installation_lists_nothing() {
        let (_dir, fnm) = installation(&[]);
        assert_eq!(fnm.installed().unwrap(), vec![]);
        assert_eq!(fnm.default_version(), None);
    }

    #[test]
    fn resolves_a_partial_version_to_the_newest_match() {
        let (_dir, fnm) = installation(&["v20.9.0", "v20.11.1", "v22.3.0"]);
        assert_eq!(fnm.resolve("20").unwrap().version, "v20.11.1");
        assert_eq!(fnm.resolve("v20.9").unwrap().version, "v20.9.0");
        assert_eq!(fnm.resolve("22.3.0").unwrap().version, "v22.3.0");

        assert!(matches!(fnm.resolve("18"), Err(Error::NotInstalled(_))));
        assert!(matches!(fnm.resolve("  "), Err(Error::InvalidVersion(_))));
        assert!(matches!(
            fnm.resolve("default"),
            Err(Error::NotInstalled(_))
        ));
    }

    #[test]
    fn the_default_is_an_alias_that_can_be_moved() {
        let (_dir, fnm) = installation(&["v20.11.1", "v22.3.0"]);

        let default = fnm.set_default("20").unwrap();
        assert_eq!(default.version, "v20.11.1");
        assert!(default.is_default);
        assert_eq!(default.aliases, vec!["default"]);
        assert_eq!(fnm.default_version().as_deref(), Some("v20.11.1"));
        assert_eq!(fnm.resolve("default").unwrap().version, "v20.11.1");

        fnm.set_default("22").unwrap();
        assert_eq!(fnm.default_version().as_deref(), Some("v22.3.0"));
        let installed = fnm.installed().unwrap();
        assert!(!installed[0].is_default);
        assert!(installed[1].is_default);
    }

    #[test]
    fn uninstalling_removes_the_version_and_its_aliases() {
        let (_dir, fnm) = installation(&["v20.9.0", "v20.11.1", "v22.3.0"]);
        fnm.set_default("22").unwrap();

        // Two versions match, so it is refused rather than guessed.
        match fnm.uninstall("20") {
            Err(Error::Ambiguous { matches, .. }) => {
                assert_eq!(matches, vec!["v20.9.0", "v20.11.1"]);
            }
            other => panic!("expected an ambiguous match, got {other:?}"),
        }

        assert_eq!(fnm.uninstall("22").unwrap(), "v22.3.0");
        assert_eq!(
            names(&fnm.installed().unwrap()),
            vec!["v20.9.0", "v20.11.1"]
        );
        assert_eq!(fnm.default_version(), None);
        assert!(matches!(fnm.uninstall("22"), Err(Error::NotInstalled(_))));
    }

    #[test]
    fn reads_the_version_a_directory_asks_for() {
        let (_dir, fnm) = installation(&[]);
        let project = tempfile::tempdir().unwrap();
        assert_eq!(fnm.requested_version(project.path()), None);

        std::fs::write(project.path().join(".node-version"), "20.11\n").unwrap();
        assert_eq!(
            fnm.requested_version(project.path()).as_deref(),
            Some("v20.11.x")
        );

        // A nested directory only inherits it when asked to look upward.
        let nested = project.path().join("packages").join("app");
        std::fs::create_dir_all(&nested).unwrap();
        assert_eq!(fnm.requested_version(&nested), None);
        let fnm = fnm.search_parent_directories(true);
        assert_eq!(fnm.requested_version(&nested).as_deref(), Some("v20.11.x"));
    }

    /// A `package.json` range can't be asked for by name, so a directory is
    /// resolved from the request itself.
    #[test]
    fn resolves_what_a_directory_asks_for_including_a_range() {
        let (_dir, fnm) = installation(&["v18.20.4", "v20.11.1", "v22.3.0"]);
        let project = tempfile::tempdir().unwrap();
        assert_eq!(fnm.resolve_for(project.path()).unwrap(), None);

        std::fs::write(
            project.path().join("package.json"),
            r#"{"engines": {"node": ">=18 <22"}}"#,
        )
        .unwrap();
        let chosen = fnm.resolve_for(project.path()).unwrap().unwrap();
        assert_eq!(chosen.version, "v20.11.1");

        // A version file takes precedence over `package.json`.
        std::fs::write(project.path().join(".nvmrc"), "18\n").unwrap();
        let chosen = fnm.resolve_for(project.path()).unwrap().unwrap();
        assert_eq!(chosen.version, "v18.20.4");

        std::fs::write(project.path().join(".nvmrc"), "16\n").unwrap();
        assert!(matches!(
            fnm.resolve_for(project.path()),
            Err(Error::NotInstalled(_))
        ));
    }

    #[test]
    fn path_for_puts_the_versions_executables_first() {
        let (_dir, fnm) = installation(&["v22.3.0"]);
        let path = fnm.path_for("22").unwrap();
        let first = std::env::split_paths(&path).next().unwrap();
        assert_eq!(first, fnm.resolve("22").unwrap().bin_dir);
    }

    #[test]
    fn reads_requests_the_way_the_command_line_does() {
        assert!(matches!(
            parse_request("lts").unwrap(),
            UserVersion::Full(Version::Lts(LtsType::Latest))
        ));
        assert!(matches!(
            parse_request("lts/iron").unwrap(),
            UserVersion::Full(Version::Lts(LtsType::CodeName(_)))
        ));
        // `latest` is the newest release when installing, and otherwise the
        // alias that installing it leaves behind.
        assert!(matches!(
            parse_install_request("Latest").unwrap(),
            UserVersion::Full(Version::Latest)
        ));
        assert!(matches!(
            parse_request("latest").unwrap(),
            UserVersion::Full(Version::Alias(_))
        ));
        assert!(matches!(
            parse_request(" 22 ").unwrap(),
            UserVersion::OnlyMajor(22)
        ));
        assert!(matches!(parse_request(""), Err(Error::InvalidVersion(_))));
    }

    /// Downloads a real release, as the downloader's own tests do.
    #[test_log::test]
    fn installs_a_version_and_makes_the_first_one_the_default() {
        let (_dir, fnm) = installation(&[]);

        let node = fnm.install("12.0.0").expect("Can't install Node 12");
        assert_eq!(node.version, "v12.0.0");
        assert!(node.is_default);
        assert_eq!(fnm.default_version().as_deref(), Some("v12.0.0"));
        let executable = node
            .bin_dir
            .join(if cfg!(windows) { "node.exe" } else { "node" });
        assert!(executable.is_file(), "{} is missing", executable.display());

        // Installing what is already there hands back the same version.
        assert_eq!(fnm.install("v12.0.0").unwrap(), node);
        assert_eq!(names(&fnm.installed().unwrap()), vec!["v12.0.0"]);

        let stdout = duct::cmd(executable.to_str().unwrap(), vec!["--version"])
            .env("PATH", fnm.path_for("12").unwrap())
            .stdout_capture()
            .run()
            .expect("Can't run Node binary")
            .stdout;
        assert_eq!(String::from_utf8(stdout).unwrap().trim(), "v12.0.0");
    }

    #[test]
    fn aliases_and_the_system_node_cant_be_installed() {
        let (_dir, fnm) = installation(&[]);
        assert!(matches!(
            fnm.install("default"),
            Err(Error::NotInstallable(_))
        ));
        assert!(matches!(
            fnm.install("system"),
            Err(Error::NotInstallable(_))
        ));
        assert!(matches!(
            Fnm::in_dir("x").with_mirror("not a url"),
            Err(Error::InvalidMirror(_))
        ));
    }
}
