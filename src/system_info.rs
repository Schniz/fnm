#[cfg(target_os = "macos")]
pub fn platform_name() -> &'static str {
    "darwin"
}

#[cfg(target_os = "linux")]
pub fn platform_name() -> &'static str {
    "linux"
}

#[cfg(target_pointer_width = "32")]
pub fn platform_arch() -> &'static str {
    "x86"
}

#[cfg(target_pointer_width = "64")]
pub fn platform_arch() -> &'static str {
    "x64"
}
