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
    let mut packages = Vec::new();
    for node_modules_dir in node_modules_dirs_for_version(version, config) {
        collect_packages_from_node_modules_dir(&node_modules_dir, &mut packages)?;
    }

    packages.sort_unstable();
    packages.dedup();

    Ok(packages)
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

fn node_modules_dirs_for_version(version: &Version, config: &FnmConfig) -> Vec<std::path::PathBuf> {
    let mut node_modules_dirs = vec![node_modules_dir_for_version(version, config)];

    if cfg!(windows) {
        if let Some(app_data) = std::env::var_os("APPDATA") {
            node_modules_dirs.push(
                std::path::PathBuf::from(app_data)
                    .join("npm")
                    .join("node_modules"),
            );
        }
    }

    node_modules_dirs
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

    #[cfg(windows)]
    #[test]
    fn test_list_for_version_reads_packages_from_appdata_npm_node_modules() {
        let base_dir = tempfile::tempdir().unwrap();
        let config = FnmConfig::default().with_base_dir(Some(base_dir.path().to_path_buf()));
        let version = Version::parse("20.11.0").unwrap();

        write_global_package(
            &config,
            &version,
            "is-odd",
            r#"{"name":"is-odd","version":"3.0.1"}"#,
        );

        let app_data = base_dir.path().join("appdata");
        let app_data_node_modules = app_data.join("npm").join("node_modules");
        std::fs::create_dir_all(app_data_node_modules.join("from-appdata")).unwrap();
        std::fs::write(
            app_data_node_modules
                .join("from-appdata")
                .join("package.json"),
            r#"{"name":"from-appdata","version":"1.0.0"}"#,
        )
        .unwrap();

        unsafe {
            std::env::set_var("APPDATA", &app_data);
        }

        let result = list_for_version(&version, &config).unwrap();

        unsafe {
            std::env::remove_var("APPDATA");
        }

        assert_eq!(result, vec!["from-appdata@1.0.0", "is-odd@3.0.1"]);
    }
}
