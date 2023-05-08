use etcetera::BaseStrategy;
use std::path::PathBuf;

#[cfg(windows)]
fn xdg_dir(env: &str) -> Option<PathBuf> {
    let env_var = std::env::var(env).ok()?;
    Some(PathBuf::from(env_var))
}

#[cfg(unix)]
fn xdg_dir(_env: &str) -> Option<PathBuf> {
    // etcetera already handles the XDG env vars on unix
    None
}

fn runtime_dir(basedirs: &impl BaseStrategy) -> Option<PathBuf> {
    xdg_dir("XDG_RUNTIME_DIR").or_else(|| basedirs.runtime_dir())
}

fn state_dir(basedirs: &impl BaseStrategy) -> Option<PathBuf> {
    xdg_dir("XDG_STATE_HOME").or_else(|| basedirs.state_dir())
}

fn cache_dir(basedirs: &impl BaseStrategy) -> PathBuf {
    xdg_dir("XDG_CACHE_HOME").unwrap_or_else(|| basedirs.cache_dir())
}

pub fn multishell_storage() -> PathBuf {
    let basedirs = etcetera::choose_base_strategy();
    let dir = if let Ok(basedirs) = &basedirs {
        runtime_dir(basedirs)
            .or_else(|| state_dir(basedirs))
            .unwrap_or_else(|| cache_dir(basedirs))
    } else {
        std::env::temp_dir()
    };
    dir.join("fnm_multishells")
}
