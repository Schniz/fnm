#[cfg(target_os = "macos")]
pub fn platform_name() -> &'static str {
    "darwin"
}

#[cfg(target_os = "linux")]
pub fn platform_name() -> &'static str {
    "linux"
}

#[cfg(all(
    target_pointer_width = "32",
    any(target_arch = "arm", target_arch = "aarch64")
))]
pub fn platform_arch() -> &'static str {
    "armv7l"
}

#[cfg(all(
    target_pointer_width = "32",
    not(any(target_arch = "arm", target_arch = "aarch64"))
))]
pub fn platform_arch() -> &'static str {
    "x86"
}

#[cfg(all(
    target_pointer_width = "64",
    any(target_arch = "arm", target_arch = "aarch64")
))]
pub fn platform_arch() -> &'static str {
    "arm64"
}

#[cfg(all(
    target_pointer_width = "64",
    not(any(target_arch = "arm", target_arch = "aarch64"))
))]
pub fn platform_arch() -> &'static str {
    "x64"
}
