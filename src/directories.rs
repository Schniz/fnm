use std::path::PathBuf;

fn xdg_dir(env: &str) -> Option<PathBuf> {
    let env_var = std::env::var(env).ok()?;
    Some(PathBuf::from(env_var))
}

fn state_dir() -> Option<PathBuf> {
    xdg_dir("XDG_STATE_HOME").or_else(dirs::state_dir)
}

fn cache_dir() -> Option<PathBuf> {
    xdg_dir("XDG_CACHE_HOME").or_else(dirs::cache_dir)
}

fn runtime_dir() -> Option<PathBuf> {
    xdg_dir("XDG_RUNTIME_DIR").or_else(dirs::runtime_dir)
}

pub fn multishell_storage() -> PathBuf {
    runtime_dir()
        .or_else(state_dir)
        .or_else(cache_dir)
        .unwrap_or_else(std::env::temp_dir)
        .join("fnm_multishells")
}
