#[cfg(target_os = "macos")]
pub fn platform_name() -> &'static str {
    "darwin"
}

#[cfg(target_os = "linux")]
pub fn platform_name() -> &'static str {
    "linux"
}

#[cfg(all(target_pointer_width = "32", target_arch = "arm"))]
pub fn platform_arch() -> &'static str {
    "armv7l"
}

#[cfg(all(target_pointer_width = "32", not(target_arch = "arm")))]
pub fn platform_arch() -> &'static str {
    "x86"
}

#[cfg(all(target_pointer_width = "64", target_arch = "arm"))]
pub fn platform_arch() -> &'static str {
    "arm64"
}

#[cfg(all(target_pointer_width = "64", not(target_arch = "arm")))]
pub fn platform_arch() -> &'static str {
    "x64"
}
