use crate::version::Version;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Arch {
    X86,
    X64,
    X64Musl,
    Arm64,
    Armv7l,
    Ppc64le,
    Ppc64,
    S390x,
}

impl Arch {
    pub fn as_str(self) -> &'static str {
        match self {
            Arch::X86 => "x86",
            Arch::X64 => "x64",
            Arch::X64Musl => "x64-musl",
            Arch::Arm64 => "arm64",
            Arch::Armv7l => "armv7l",
            Arch::Ppc64le => "ppc64le",
            Arch::Ppc64 => "ppc64",
            Arch::S390x => "s390x",
        }
    }
}

#[cfg(unix)]
/// handle common case: Apple Silicon / Node < 16
pub fn get_safe_arch(arch: Arch, version: &Version) -> Arch {
    use crate::system_info::{platform_arch, platform_name};

    match (platform_name(), platform_arch(), version) {
        ("darwin", "arm64", Version::Semver(v)) if v.major < 16 => Arch::X64,
        _ => arch,
    }
}

#[cfg(windows)]
/// handle common case: Apple Silicon / Node < 16
pub fn get_safe_arch(arch: Arch, _version: &Version) -> Arch {
    arch
}

impl Default for Arch {
    fn default() -> Arch {
        match crate::system_info::platform_arch().parse() {
            Ok(arch) => arch,
            Err(e) => panic!("{}", e.details),
        }
    }
}

impl std::str::FromStr for Arch {
    type Err = ArchError;
    fn from_str(s: &str) -> Result<Arch, Self::Err> {
        match s {
            "x86" => Ok(Arch::X86),
            "x64" => Ok(Arch::X64),
            "x64-musl" => Ok(Arch::X64Musl),
            "arm64" => Ok(Arch::Arm64),
            "armv7l" => Ok(Arch::Armv7l),
            "ppc64le" => Ok(Arch::Ppc64le),
            "ppc64" => Ok(Arch::Ppc64),
            "s390x" => Ok(Arch::S390x),
            unknown => Err(ArchError::new(format!("Unknown Arch: {unknown}"))),
        }
    }
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug)]
pub struct ArchError {
    details: String,
}

impl ArchError {
    fn new(msg: String) -> ArchError {
        ArchError { details: msg }
    }
}

impl std::fmt::Display for ArchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.details)
    }
}

impl std::error::Error for ArchError {
    fn description(&self) -> &str {
        &self.details
    }
}
