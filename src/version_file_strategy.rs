use clap::ValueEnum;
use std::fmt::Display;

#[derive(Debug, Clone, Default, ValueEnum)]
pub enum VersionFileStrategy {
    /// Use the local version of Node defined within the current directory
    #[default]
    Local,
    /// Use the version of Node defined within the current directory and all parent directories
    Recursive,
}

impl Display for VersionFileStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionFileStrategy::Local => write!(f, "local"),
            VersionFileStrategy::Recursive => write!(f, "recursive"),
        }
    }
}

impl VersionFileStrategy {
    pub fn as_str(&self) -> &'static str {
        match self {
            VersionFileStrategy::Local => "local",
            VersionFileStrategy::Recursive => "recursive",
        }
    }
}
