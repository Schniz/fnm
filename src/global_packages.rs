use crate::config::FnmConfig;
use crate::version::Version;
use log::warn;
use std::path::Path;

#[derive(serde::Deserialize)]
struct NpmPackageManifest {
    name: String,
    version: String,
}

pub fn list_for_version(version: &Version, config: &FnmConfig) -> std::io::Result<Vec<String>> {
    let node_modules_dir = node_modules_dir_for_version(version, config);
    if !node_modules_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut packages = Vec::new();
    for entry in std::fs::read_dir(&node_modules_dir)? {
        let entry = entry?;
        let path = entry.path();
        let package_name = entry.file_name();
        let package_name = package_name.to_string_lossy();

        if package_name.starts_with('@') {
            for scoped_entry in std::fs::read_dir(&path)? {
                let scoped_entry = scoped_entry?;
                if let Some(spec) = package_spec_from_dir(&scoped_entry.path())? {
                    packages.push(spec);
                }
            }
            continue;
        }

        if let Some(spec) = package_spec_from_dir(&path)? {
            packages.push(spec);
        }
    }

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
}
