use std::str::FromStr;

#[derive(Debug)]
pub enum VersionSwitchStrategy {
    /// Creates a binary shim for calling `node`, instead of invoking it directly.
    /// This is a WIP feature, and is not recommended for general use (unless one
    /// wants to help with the development of this feature).
    Shims,

    /// This is the default strategy. It creates a symlink to the Node.js binary
    /// in the `bin` directory of the Node.js installation.
    /// Then, it sets the `PATH` environment variable to point to that directory.
    /// This is the most compatible strategy, but it requires rehashing the shell
    /// every time you change the Node.js version.
    PathSymlink,
}

impl VersionSwitchStrategy {
    pub fn possible_values() -> &'static [&'static str] {
        &["shims", "path-symlink"]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Shims => "shims",
            Self::PathSymlink => "path-symlink",
        }
    }
}

impl FromStr for VersionSwitchStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "shims" => Ok(VersionSwitchStrategy::Shims),
            "path-symlink" => Ok(VersionSwitchStrategy::PathSymlink),
            _ => Err(format!(
                "Unknown version switch strategy: {}. Valid values are: shims, path-symlink",
                s
            )),
        }
    }
}

impl Default for VersionSwitchStrategy {
    fn default() -> Self {
        VersionSwitchStrategy::PathSymlink
    }
}
