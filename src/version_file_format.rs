use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum VersionFileFormat {
    Nvmrc,
    PackageJson,
    Other,
}

impl VersionFileFormat {
    pub fn infer_from_path(path: &Path) -> Self {
        match path.file_name().and_then(|name| name.to_str()) {
            Some(".nvmrc") => Self::Nvmrc,
            Some("package.json") => Self::PackageJson,
            _ => Self::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_infer_nvmrc() {
        let path = PathBuf::from("/some/path/.nvmrc");
        assert_eq!(
            VersionFileFormat::infer_from_path(&path),
            VersionFileFormat::Nvmrc
        );
    }

    #[test]
    fn test_infer_package_json() {
        let path = PathBuf::from("/some/path/package.json");
        assert_eq!(
            VersionFileFormat::infer_from_path(&path),
            VersionFileFormat::PackageJson
        );
    }

    #[test]
    fn test_infer_node_version() {
        let path = PathBuf::from("/some/path/.node-version");
        assert_eq!(
            VersionFileFormat::infer_from_path(&path),
            VersionFileFormat::Other
        );
    }

    #[test]
    fn test_infer_unknown() {
        let path = PathBuf::from("/some/path/unknown.txt");
        assert_eq!(
            VersionFileFormat::infer_from_path(&path),
            VersionFileFormat::Other
        );
    }
}
