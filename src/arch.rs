use crate::version::Version;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Arch(String);

impl Arch {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn x64() -> Self {
        Self::new("x64")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(unix)]
/// handle common case: Apple Silicon / Node < 16
pub fn get_safe_arch(arch: &Arch, version: &Version) -> Arch {
    use crate::system_info::{platform_arch, platform_name};

    match (platform_name(), platform_arch(), version) {
        ("darwin", "arm64", Version::Semver(v)) if v.major < 16 => Arch::x64(),
        _ => arch.clone(),
    }
}

#[cfg(windows)]
/// handle common case: Apple Silicon / Node < 16
pub fn get_safe_arch(arch: &Arch, _version: &Version) -> Arch {
    arch.clone()
}

impl Default for Arch {
    fn default() -> Arch {
        Arch::new(crate::system_info::platform_arch())
    }
}

impl std::str::FromStr for Arch {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Arch, Self::Err> {
        Ok(Arch::new(s))
    }
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
