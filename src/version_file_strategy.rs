use std::str::FromStr;

#[derive(Debug)]
pub enum VersionFileStrategy {
    Local,
    Recursive,
}

impl VersionFileStrategy {
    pub fn possible_values() -> &'static [&'static str] {
        &["local", "recursive"]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            VersionFileStrategy::Local => "local",
            VersionFileStrategy::Recursive => "recursive",
        }
    }
}

impl Default for VersionFileStrategy {
    fn default() -> Self {
        VersionFileStrategy::Local
    }
}

impl FromStr for VersionFileStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "local" => Ok(VersionFileStrategy::Local),
            "recursive" => Ok(VersionFileStrategy::Recursive),
            _ => Err(format!(
                "Invalid strategy: {}. Expected one of: local, recursive",
                s
            )),
        }
    }
}
