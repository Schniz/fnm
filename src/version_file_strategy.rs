use clap::ValueEnum;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum VersionFileStrategy {
    /// Use the local version of Node defined within the current directory
    #[default]
    Local,
    /// Use the version of Node defined within the current directory and all parent directories
    Recursive,
}

impl VersionFileStrategy {
    pub fn as_str(self) -> &'static str {
        match self {
            VersionFileStrategy::Local => "local",
            VersionFileStrategy::Recursive => "recursive",
        }
    }
}

impl Display for VersionFileStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
