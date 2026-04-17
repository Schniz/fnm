use super::command::Command;
use super::r#use::Use;
use crate::alias::create_alias;
use crate::arch::get_safe_arch;
use crate::config::FnmConfig;
use crate::downloader::{install_node_dist, Error as DownloaderError};
use crate::installed_versions;
use crate::lts::LtsType;
use crate::outln;
use crate::progress::ProgressConfig;
use crate::remote_node_index;
use crate::user_version::UserVersion;
use crate::user_version_reader::UserVersionReader;
use crate::version::Version;
use crate::version_files::get_user_version_for_directory;
use colored::Colorize;
use log::debug;
use thiserror::Error;

#[derive(clap::Parser, Debug, Default)]
pub struct Install {
    /// A version string. Can be a partial semver or a LTS version name by the format lts/NAME
    pub version: Option<UserVersion>,

    /// Install latest LTS
    #[clap(long, conflicts_with_all = &["version", "latest"])]
    pub lts: bool,

    /// Install latest version
    #[clap(long, conflicts_with_all = &["version", "lts"])]
    pub latest: bool,

    /// Show an interactive progress bar for the download
    /// status.
    #[clap(long, default_value_t)]
    #[arg(value_enum)]
    pub progress: ProgressConfig,

    /// Use the installed version immediately after installation
    #[clap(long)]
    pub r#use: bool,

    /// Reinstall global packages from a specified Node version after installing.
    /// Analogous to nvm's --reinstall-packages-from flag.
    #[clap(long, value_name = "version")]
    pub reinstall_packages_from: Option<UserVersion>,
}

impl Install {
    fn version(self) -> Result<Option<UserVersion>, Error> {
        match self {
            Self {
                version: v,
                lts: false,
                latest: false,
                ..
            } => Ok(v),
            Self {
                version: None,
                lts: true,
                latest: false,
                ..
            } => Ok(Some(UserVersion::Full(Version::Lts(LtsType::Latest)))),
            Self {
                version: None,
                lts: false,
                latest: true,
                ..
            } => Ok(Some(UserVersion::Full(Version::Latest))),
            _ => Err(Error::TooManyVersionsProvided),
        }
    }
}

impl Command for Install {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let current_dir = std::env::current_dir().unwrap();
        let show_progress = self.progress.enabled(config);
        let use_installed = self.r#use;
        let reinstall_packages_from = self.reinstall_packages_from.clone();

        let current_version = self
            .version()?
            .or_else(|| get_user_version_for_directory(current_dir, config))
            .ok_or(Error::CantInferVersion)?;

        let version = match current_version.clone() {
            UserVersion::Full(Version::Semver(actual_version)) => Version::Semver(actual_version),
            UserVersion::Full(v @ (Version::Bypassed | Version::Alias(_))) => {
                return Err(Error::UninstallableVersion { version: v });
            }
            UserVersion::Full(Version::Lts(lts_type)) => {
                let available_versions: Vec<_> = remote_node_index::list(&config.node_dist_mirror)
                    .map_err(|source| Error::CantListRemoteVersions { source })?;
                let picked_version = lts_type
                    .pick_latest(&available_versions)
                    .ok_or_else(|| Error::CantFindRelevantLts {
                        lts_type: lts_type.clone(),
                    })?
                    .version
                    .clone();
                debug!(
                    "Resolved {} into Node version {}",
                    Version::Lts(lts_type).v_str().cyan(),
                    picked_version.v_str().cyan()
                );
                picked_version
            }
            UserVersion::Full(Version::Latest) => {
                let available_versions: Vec<_> = remote_node_index::list(&config.node_dist_mirror)
                    .map_err(|source| Error::CantListRemoteVersions { source })?;
                let picked_version = available_versions
                    .last()
                    .ok_or(Error::CantFindLatest)?
                    .version
                    .clone();
                debug!(
                    "Resolved {} into Node version {}",
                    Version::Latest.v_str().cyan(),
                    picked_version.v_str().cyan()
                );
                picked_version
            }
            current_version => {
                let available_versions: Vec<_> = remote_node_index::list(&config.node_dist_mirror)
                    .map_err(|source| Error::CantListRemoteVersions { source })?
                    .drain(..)
                    .map(|x| x.version)
                    .collect();

                current_version
                    .to_version(&available_versions, config)
                    .ok_or(Error::CantFindNodeVersion {
                        requested_version: current_version,
                    })?
                    .clone()
            }
        };

        // Automatically swap Apple Silicon to x64 arch for appropriate versions.
        let safe_arch = get_safe_arch(config.arch, &version);

        let version_str = format!("Node {}", &version);
        outln!(
            config,
            Info,
            "Installing {} ({})",
            version_str.cyan(),
            safe_arch.as_str()
        );

        match install_node_dist(
            &version,
            &config.node_dist_mirror,
            config.installations_dir(),
            safe_arch,
            show_progress,
        ) {
            Err(err @ DownloaderError::VersionAlreadyInstalled { .. }) => {
                outln!(config, Error, "{} {}", "warning:".bold().yellow(), err);
            }
            Err(source) => Err(Error::DownloadError { source })?,
            Ok(()) => {}
        }

        if !config.default_version_dir().exists() {
            debug!("Tagging {} as the default version", version.v_str().cyan());
            create_alias(config, "default", &version)?;
        }

        if let Some(tagged_alias) = current_version.inferred_alias() {
            tag_alias(config, &version, &tagged_alias)?;
        }

        if config.corepack_enabled() {
            outln!(config, Info, "Enabling corepack for {}", version_str.cyan());
            enable_corepack(&version, config)?;
        }

        if let Some(source_version_str) = reinstall_packages_from {
            reinstall_packages_from_version(&source_version_str, &version, config)?;
        }

        if use_installed {
            use_installed_version(&version, config)?;
        }

        Ok(())
    }
}

fn tag_alias(config: &FnmConfig, matched_version: &Version, alias: &Version) -> Result<(), Error> {
    let alias_name = alias.v_str();
    debug!(
        "Tagging {} as alias for {}",
        alias_name.cyan(),
        matched_version.v_str().cyan()
    );
    create_alias(config, &alias_name, matched_version)?;

    Ok(())
}

fn enable_corepack(version: &Version, config: &FnmConfig) -> Result<(), Error> {
    let corepack_path = version.installation_path(config);
    let corepack_path = if cfg!(windows) {
        corepack_path.join("corepack.cmd")
    } else {
        corepack_path.join("bin").join("corepack")
    };
    super::exec::Exec::new_for_version(version, corepack_path.to_str().unwrap(), &["enable"])
        .apply(config)
        .map_err(|source| Error::CorepackError { source })?;
    Ok(())
}

fn reinstall_packages_from_version(
    source_version_str: &UserVersion,
    target_version: &Version,
    config: &FnmConfig,
) -> Result<(), Error> {
    let all_versions = installed_versions::list(config.installations_dir()).map_err(|source| {
        Error::ReinstallPackagesError {
            source: Box::new(source),
        }
    })?;
    let source_version = source_version_str
        .to_version(&all_versions, config)
        .ok_or_else(|| Error::ReinstallPackagesFromVersionNotInstalled {
            version: source_version_str.clone(),
        })?
        .clone();

    if source_version == *target_version {
        outln!(
            config,
            Info,
            "Source and target versions are the same ({}). Skipping package reinstallation.",
            format!("Node {source_version}").cyan()
        );
        return Ok(());
    }

    let packages = list_global_packages(&source_version, config)?;
    let source_version_display = format!("Node {source_version}");
    if packages.is_empty() {
        outln!(
            config,
            Info,
            "No global packages found in {}.",
            source_version_display.cyan()
        );
        return Ok(());
    }

    outln!(
        config,
        Info,
        "Reinstalling global packages from {}...",
        source_version_display.cyan()
    );
    for package in &packages {
        outln!(config, Info, "  - {}", package);
    }
    reinstall_packages(&packages, target_version, config)?;
    outln!(
        config,
        Info,
        "Successfully reinstalled {} packages.",
        packages.len()
    );

    Ok(())
}

/// Returns the npm binary path and a PATH env value with the version's bin dir prepended.
fn npm_env_for_version(
    version: &Version,
    config: &FnmConfig,
) -> Result<(std::path::PathBuf, std::ffi::OsString), Error> {
    let installation_path = version.installation_path(config);
    let (npm_path, bin_dir) = if cfg!(windows) {
        (installation_path.join("npm.cmd"), installation_path)
    } else {
        (
            installation_path.join("bin").join("npm"),
            installation_path.join("bin"),
        )
    };
    let path_env =
        prepend_to_path_env(bin_dir).map_err(|source| Error::ReinstallPackagesError {
            source: Box::new(source),
        })?;
    Ok((npm_path, path_env))
}

fn list_global_packages(version: &Version, config: &FnmConfig) -> Result<Vec<String>, Error> {
    use std::process::Command as StdCommand;

    let (npm_path, path_env) = npm_env_for_version(version, config)?;

    let output = StdCommand::new(&npm_path)
        .args([
            "ls",
            "--global",
            "--parseable",
            "--long",
            "--loglevel=error",
        ])
        .env("PATH", path_env)
        .output()
        .map_err(|source| Error::ReinstallPackagesError {
            source: Box::new(source),
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !output.status.success() && stdout.trim().is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::ReinstallPackagesError {
            source: std::io::Error::other(format!(
                "npm ls exited with {:?}: {}",
                output.status,
                stderr.trim()
            ))
            .into(),
        });
    }

    Ok(parse_npm_ls_global_parseable_long_output(&stdout))
}

fn reinstall_packages(
    packages: &[String],
    target_version: &Version,
    config: &FnmConfig,
) -> Result<(), Error> {
    use std::process::{Command as StdCommand, Stdio};

    if packages.is_empty() {
        return Ok(());
    }

    let (npm_path, path_env) = npm_env_for_version(target_version, config)?;

    let status = StdCommand::new(&npm_path)
        .args(["install", "--global"])
        .args(packages)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .env("PATH", path_env)
        .status()
        .map_err(|source| Error::ReinstallPackagesError {
            source: Box::new(source),
        })?;

    if !status.success() {
        return Err(Error::ReinstallPackagesError {
            source: std::io::Error::other(format!("npm install exited with {status:?}")).into(),
        });
    }

    Ok(())
}

fn prepend_to_path_env(
    bin_dir: std::path::PathBuf,
) -> Result<std::ffi::OsString, std::env::JoinPathsError> {
    let mut paths: Vec<std::path::PathBuf> = std::env::var_os("PATH")
        .map(|paths_env| std::env::split_paths(&paths_env).collect())
        .unwrap_or_default();
    paths.insert(0, bin_dir);
    std::env::join_paths(paths)
}

fn parse_npm_ls_global_parseable_long_output(output: &str) -> Vec<String> {
    output
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let (_path, package_spec) = line.trim_end_matches(':').rsplit_once(':')?;
            let package_spec = package_spec.trim();
            if package_spec.is_empty() {
                return None;
            }

            if package_spec.starts_with("npm@") || package_spec.starts_with("corepack@") {
                return None;
            }

            let version_separator_index = package_spec.rfind('@')?;
            if version_separator_index == 0 || version_separator_index == package_spec.len() - 1 {
                return None;
            }

            Some(package_spec.to_string())
        })
        .collect()
}

fn use_installed_version(version: &Version, config: &FnmConfig) -> Result<(), Error> {
    Use {
        version: Some(UserVersionReader::Direct(UserVersion::Full(
            version.clone(),
        ))),
        install_if_missing: false,
        silent_if_unchanged: false,
        info_to_stderr: false,
    }
    .apply(config)
    .map_err(|source| Error::UseError {
        source: Box::new(source),
    })?;
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't download the requested binary: {}", source)]
    DownloadError { source: DownloaderError },
    #[error(transparent)]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error("Can't enable corepack: {source}")]
    CorepackError {
        #[from]
        source: super::exec::Error,
    },
    #[error(transparent)]
    UseError {
        source: Box<<Use as Command>::Error>,
    },
    #[error("Can't find version in dotfiles. Please provide a version manually to the command.")]
    CantInferVersion,
    #[error(transparent)]
    CantListRemoteVersions { source: remote_node_index::Error },
    #[error(
        "Can't find a Node version that matches {} in remote",
        requested_version
    )]
    CantFindNodeVersion { requested_version: UserVersion },
    #[error("Can't find relevant LTS named {}", lts_type)]
    CantFindRelevantLts { lts_type: crate::lts::LtsType },
    #[error("Can't find any versions in the upstream version index.")]
    CantFindLatest,
    #[error("The requested version is not installable: {}", version.v_str())]
    UninstallableVersion { version: Version },
    #[error("Too many versions provided. Please don't use --lts with a version string.")]
    TooManyVersionsProvided,
    #[error("Version {version} is not installed. Install it first with 'fnm install {version}'.")]
    ReinstallPackagesFromVersionNotInstalled { version: UserVersion },
    #[error("Failed to reinstall packages: {source}")]
    ReinstallPackagesError {
        #[source]
        source: Box<dyn std::error::Error>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn test_set_default_on_new_installation() {
        let base_dir = tempfile::tempdir().unwrap();
        let config = FnmConfig::default().with_base_dir(Some(base_dir.path().to_path_buf()));
        assert!(!config.default_version_dir().exists());

        Install {
            version: UserVersion::from_str("12.0.0").ok(),
            lts: false,
            latest: false,
            progress: ProgressConfig::Never,
            r#use: false,
            reinstall_packages_from: None,
        }
        .apply(&config)
        .expect("Can't install");

        assert!(config.default_version_dir().exists());
        assert_eq!(
            config.default_version_dir().canonicalize().ok(),
            config
                .installations_dir()
                .join("v12.0.0")
                .join("installation")
                .canonicalize()
                .ok()
        );
    }

    #[test]
    fn test_install_latest() {
        let base_dir = tempfile::tempdir().unwrap();
        let config = FnmConfig::default().with_base_dir(Some(base_dir.path().to_path_buf()));

        Install {
            version: None,
            lts: false,
            latest: true,
            progress: ProgressConfig::Never,
            r#use: false,
            reinstall_packages_from: None,
        }
        .apply(&config)
        .expect("Can't install");

        let available_versions: Vec<_> =
            remote_node_index::list(&config.node_dist_mirror).expect("Can't get node version list");
        let latest_version = available_versions.last().unwrap().version.clone();

        assert!(config.installations_dir().exists());
        assert!(config
            .installations_dir()
            .join(latest_version.to_string())
            .join("installation")
            .canonicalize()
            .unwrap()
            .exists());
    }

    #[test]
    fn test_parse_npm_ls_global_parseable_long_output() {
        let output = r"
/path/to/lib
/path/to/node_modules/typescript:typescript@5.4.2:
/path/to/node_modules/eslint:eslint@9.0.0:
C:\Users\me\node_modules\prettier:prettier@3.2.5:
/path/to/node_modules/@openai/codex:@openai/codex@0.99.0:
        ";

        let result = parse_npm_ls_global_parseable_long_output(output);
        assert_eq!(
            result,
            vec![
                "typescript@5.4.2".to_string(),
                "eslint@9.0.0".to_string(),
                "prettier@3.2.5".to_string(),
                "@openai/codex@0.99.0".to_string(),
            ]
        );
    }

    #[test]
    fn test_parse_npm_ls_global_parseable_long_output_filters_builtins() {
        let output = r"
/path/to/node_modules/npm:npm@10.0.0:
/path/to/node_modules/corepack:corepack@0.28.0:
/path/to/node_modules/is-odd:is-odd@3.0.1:
        ";

        let result = parse_npm_ls_global_parseable_long_output(output);
        assert_eq!(result, vec!["is-odd@3.0.1".to_string()]);
    }

    #[test]
    fn test_parse_npm_ls_global_parseable_long_output_empty() {
        let result = parse_npm_ls_global_parseable_long_output("");
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_parse_npm_ls_global_parseable_long_output_skips_malformed_lines() {
        let output = r"
this is not parseable output
/path/to/node_modules/is-odd:is-odd@3.0.1:
        ";

        let result = parse_npm_ls_global_parseable_long_output(output);
        assert_eq!(result, vec!["is-odd@3.0.1".to_string()]);
    }
}
