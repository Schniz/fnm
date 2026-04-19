use crate::config::FnmConfig;
use crate::version::Version;
use log::warn;
use std::collections::HashMap;
use std::path::Path;

#[derive(serde::Deserialize)]
struct NpmPackageManifest {
    name: String,
    version: String,
}

#[derive(serde::Deserialize)]
struct NpmLsRoot {
    #[serde(default)]
    dependencies: HashMap<String, NpmLsPackage>,
}

#[derive(serde::Deserialize)]
struct NpmLsPackage {
    version: Option<String>,
}

pub fn list_for_version(version: &Version, config: &FnmConfig) -> std::io::Result<Vec<String>> {
    // On Windows, npm global installs are commonly resolved via npm's configured prefix
    // (often under %APPDATA%\npm), which is not reliably derivable from fnm's Node
    // installation path alone. Use npm ls for source-version discovery there.
    if cfg!(windows) {
        return list_for_version_with_npm_ls(version, config);
    }

    let mut packages = Vec::new();
    let version_node_modules_dir = node_modules_dir_for_version(version, config);
    collect_packages_from_node_modules_dir(&version_node_modules_dir, &mut packages)?;

    packages.sort_unstable();
    packages.dedup();

    Ok(packages)
}

fn list_for_version_with_npm_ls(
    version: &Version,
    config: &FnmConfig,
) -> std::io::Result<Vec<String>> {
    let npm_path = version.installation_path(config).join("npm.cmd");
    let output = std::process::Command::new(&npm_path)
        .args(["ls", "--global", "--depth=0", "--json", "--loglevel=error"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !output.status.success() && stdout.trim().is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::other(format!(
            "npm ls exited with {:?}: {}",
            output.status,
            stderr.trim()
        )));
    }

    if !output.status.success() {
        warn!(
            "npm ls exited with {:?} but produced output; proceeding with partial package list",
            output.status
        );
    }

    parse_npm_ls_global_json_output(&stdout)
}

fn collect_packages_from_node_modules_dir(
    node_modules_dir: &Path,
    packages: &mut Vec<String>,
) -> std::io::Result<()> {
    if !node_modules_dir.is_dir() {
        return Ok(());
    }

    for entry in std::fs::read_dir(node_modules_dir)? {
        let entry = entry?;
        let path = entry.path();
        let package_name = entry.file_name();
        let package_name = package_name.to_string_lossy();

        if is_symlink(&path)? {
            warn!(
                "Skipping symlinked global package entry {} at {}",
                package_name,
                path.to_string_lossy()
            );
            continue;
        }

        if package_name.starts_with('@') {
            for scoped_entry in std::fs::read_dir(&path)? {
                let scoped_entry = scoped_entry?;
                let scoped_path = scoped_entry.path();
                let scoped_package_name = scoped_entry.file_name();
                let scoped_package_name = scoped_package_name.to_string_lossy();

                if is_symlink(&scoped_path)? {
                    warn!(
                        "Skipping symlinked global package entry {}/{} at {}",
                        package_name,
                        scoped_package_name,
                        scoped_path.to_string_lossy()
                    );
                    continue;
                }

                if let Some(spec) = package_spec_from_dir(&scoped_path)? {
                    packages.push(spec);
                }
            }
            continue;
        }

        if let Some(spec) = package_spec_from_dir(&path)? {
            packages.push(spec);
        }
    }

    Ok(())
}

fn is_symlink(path: &Path) -> std::io::Result<bool> {
    Ok(std::fs::symlink_metadata(path)?.file_type().is_symlink())
}

fn parse_npm_ls_global_json_output(stdout: &str) -> std::io::Result<Vec<String>> {
    let npm_ls: NpmLsRoot = serde_json::from_str(stdout).map_err(std::io::Error::other)?;

    let mut packages = npm_ls
        .dependencies
        .into_iter()
        .filter_map(|(name, package)| {
            if name == "npm" || name == "corepack" {
                return None;
            }

            let version = package.version?;
            if version.trim().is_empty() {
                return None;
            }

            Some(format!("{}@{}", name, version))
        })
        .collect::<Vec<_>>();

    packages.sort_unstable();
    packages.dedup();

    Ok(packages)
}

pub fn node_modules_dir_for_version(version: &Version, config: &FnmConfig) -> std::path::PathBuf {
    let installation_path = version.installation_path(config);
    if cfg!(windows) {
        installation_path.join("node_modules")
    } else {
        installation_path.join("lib").join("node_modules")
    }
}

fn package_spec_from_dir(package_dir: &Path) -> std::io::Result<Option<String>> {
    if !package_dir.is_dir() {
        return Ok(None);
    }

    let manifest_path = package_dir.join("package.json");
    if !manifest_path.is_file() {
        return Ok(None);
    }

    let manifest = std::fs::read_to_string(&manifest_path)?;
    let package: NpmPackageManifest = match serde_json::from_str(&manifest) {
        Ok(package) => package,
        Err(source) => {
            warn!(
                "Failed to parse {}: {source}",
                manifest_path.to_string_lossy()
            );
            return Ok(None);
        }
    };

    if package.name == "npm" || package.name == "corepack" {
        return Ok(None);
    }

    if package.name.trim().is_empty() || package.version.trim().is_empty() {
        warn!(
            "Skipping package with missing name/version in {}",
            manifest_path.to_string_lossy()
        );
        return Ok(None);
    }

    Ok(Some(format!("{}@{}", package.name, package.version)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn write_global_package(
        config: &FnmConfig,
        version: &Version,
        package_path: &str,
        manifest: &str,
    ) {
        let package_dir = node_modules_dir_for_version(version, config).join(package_path);
        std::fs::create_dir_all(&package_dir).unwrap();
        std::fs::write(package_dir.join("package.json"), manifest).unwrap();
    }

    #[test]
    fn test_node_modules_dir_for_version() {
        let config = FnmConfig::default();
        let version = Version::parse("20.11.0").unwrap();

        let global_dir = node_modules_dir_for_version(&version, &config);
        if cfg!(windows) {
            assert!(global_dir.ends_with("installation/node_modules"));
        } else {
            assert!(global_dir.ends_with("installation/lib/node_modules"));
        }
    }

    #[test]
    fn test_list_for_version_reads_unscoped_and_scoped_packages() {
        let base_dir = tempfile::tempdir().unwrap();
        let config = FnmConfig::default().with_base_dir(Some(base_dir.path().to_path_buf()));
        let version = Version::parse("20.11.0").unwrap();

        write_global_package(
            &config,
            &version,
            "is-odd",
            r#"{"name":"is-odd","version":"3.0.1"}"#,
        );
        write_global_package(
            &config,
            &version,
            "@scope/tool",
            r#"{"name":"@scope/tool","version":"1.2.3"}"#,
        );
        write_global_package(
            &config,
            &version,
            "npm",
            r#"{"name":"npm","version":"10.0.0"}"#,
        );
        write_global_package(
            &config,
            &version,
            "corepack",
            r#"{"name":"corepack","version":"0.28.0"}"#,
        );

        let result = list_for_version(&version, &config).unwrap();
        assert_eq!(result, vec!["@scope/tool@1.2.3", "is-odd@3.0.1"]);
    }

    #[test]
    fn test_list_for_version_returns_empty_when_node_modules_is_missing() {
        let base_dir = tempfile::tempdir().unwrap();
        let config = FnmConfig::default().with_base_dir(Some(base_dir.path().to_path_buf()));
        let version = Version::parse("20.11.0").unwrap();

        let result = list_for_version(&version, &config).unwrap();
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_list_for_version_skips_malformed_package_json() {
        let base_dir = tempfile::tempdir().unwrap();
        let config = FnmConfig::default().with_base_dir(Some(base_dir.path().to_path_buf()));
        let version = Version::parse("20.11.0").unwrap();

        write_global_package(&config, &version, "is-even", "this is not valid json");
        write_global_package(
            &config,
            &version,
            "is-odd",
            r#"{"name":"is-odd","version":"3.0.1"}"#,
        );

        let result = list_for_version(&version, &config).unwrap();
        assert_eq!(result, vec!["is-odd@3.0.1"]);
    }

    #[test]
    fn test_list_for_version_skips_directories_without_package_json() {
        let base_dir = tempfile::tempdir().unwrap();
        let config = FnmConfig::default().with_base_dir(Some(base_dir.path().to_path_buf()));
        let version = Version::parse("20.11.0").unwrap();

        let package_dir = node_modules_dir_for_version(&version, &config).join("left-pad");
        std::fs::create_dir_all(package_dir).unwrap();

        let result = list_for_version(&version, &config).unwrap();
        assert_eq!(result, Vec::<String>::new());
    }

    #[cfg(unix)]
    #[test]
    fn test_list_for_version_skips_symlinked_packages() {
        use std::os::unix::fs::symlink;

        let base_dir = tempfile::tempdir().unwrap();
        let config = FnmConfig::default().with_base_dir(Some(base_dir.path().to_path_buf()));
        let version = Version::parse("20.11.0").unwrap();

        write_global_package(
            &config,
            &version,
            "is-odd",
            r#"{"name":"is-odd","version":"3.0.1"}"#,
        );

        let external_package = base_dir.path().join("linked-package");
        std::fs::create_dir_all(&external_package).unwrap();
        std::fs::write(
            external_package.join("package.json"),
            r#"{"name":"linked-only","version":"1.0.0"}"#,
        )
        .unwrap();

        let node_modules_dir = node_modules_dir_for_version(&version, &config);
        std::fs::create_dir_all(&node_modules_dir).unwrap();
        symlink(&external_package, node_modules_dir.join("linked-only")).unwrap();

        let result = list_for_version(&version, &config).unwrap();
        assert_eq!(result, vec!["is-odd@3.0.1"]);
    }

    #[test]
    fn test_parse_npm_ls_global_json_output() {
        let result = parse_npm_ls_global_json_output(
            r#"{
                "dependencies": {
                    "is-odd": { "version": "3.0.1" },
                    "@scope/tool": { "version": "1.2.3" },
                    "npm": { "version": "10.9.0" },
                    "corepack": { "version": "0.29.4" }
                }
            }"#,
        )
        .unwrap();

        assert_eq!(result, vec!["@scope/tool@1.2.3", "is-odd@3.0.1"]);
    }

    #[test]
    fn test_parse_npm_ls_global_json_output_skips_missing_version() {
        let result = parse_npm_ls_global_json_output(
            r#"{
                "dependencies": {
                    "is-odd": { "version": "3.0.1" },
                    "broken": {}
                }
            }"#,
        )
        .unwrap();

        assert_eq!(result, vec!["is-odd@3.0.1"]);
    }
}
