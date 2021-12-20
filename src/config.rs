use crate::arch::Arch;
use crate::log_level::LogLevel;
use crate::outln;
use crate::path_ext::PathExt;
use colored::Colorize;
use dirs::{data_dir, home_dir};
use std::sync::atomic::{AtomicBool, Ordering};
use structopt::StructOpt;
use url::Url;

static HAS_WARNED_DEPRECATED_BASE_DIR: AtomicBool = AtomicBool::new(false);

#[derive(StructOpt, Debug)]
pub struct FnmConfig {
    /// https://nodejs.org/dist/ mirror
    #[structopt(
        long,
        env = "FNM_NODE_DIST_MIRROR",
        default_value = "https://nodejs.org/dist",
        global = true,
        hide_env_values = true
    )]
    pub node_dist_mirror: Url,

    /// The root directory of fnm installations.
    #[structopt(
        long = "fnm-dir",
        env = "FNM_DIR",
        global = true,
        hide_env_values = true
    )]
    pub base_dir: Option<std::path::PathBuf>,

    /// Where the current node version link is stored.
    /// This value will be populated automatically by evaluating
    /// `fnm env` in your shell profile. Read more about it using `fnm help env`
    #[structopt(
        long,
        env = "FNM_MULTISHELL_PATH",
        hide_env_values = true,
        hidden = true
    )]
    multishell_path: Option<std::path::PathBuf>,

    /// The log level of fnm commands
    #[structopt(
        long,
        env = "FNM_LOGLEVEL",
        default_value = "info",
        global = true,
        hide_env_values = true,
        possible_values = LogLevel::possible_values()
    )]
    log_level: LogLevel,

    /// Override the architecture of the installed Node binary.
    /// Defaults to arch of fnm binary.
    #[structopt(
        long,
        env = "FNM_ARCH",
        default_value,
        global = true,
        hide_env_values = true
    )]
    pub arch: Arch,
}

impl Default for FnmConfig {
    fn default() -> Self {
        Self {
            node_dist_mirror: Url::parse("https://nodejs.org/dist/").unwrap(),
            base_dir: None,
            multishell_path: None,
            log_level: LogLevel::Info,
            arch: Arch::default(),
        }
    }
}

impl FnmConfig {
    pub fn multishell_path(&self) -> Option<&std::path::Path> {
        match &self.multishell_path {
            None => None,
            Some(v) => Some(v.as_path()),
        }
    }

    pub fn log_level(&self) -> &LogLevel {
        &self.log_level
    }

    pub fn base_dir_with_default(&self) -> std::path::PathBuf {
        let user_pref = self.base_dir.clone();
        if let Some(dir) = user_pref {
            return dir;
        }

        let legacy = home_dir()
            .map(|dir| dir.join(".fnm"))
            .filter(|dir| dir.exists());

        let modern = data_dir().map(|dir| dir.join("fnm"));

        if let Some(dir) = legacy {
            if !HAS_WARNED_DEPRECATED_BASE_DIR.load(Ordering::SeqCst) {
                HAS_WARNED_DEPRECATED_BASE_DIR.store(true, Ordering::SeqCst);

                let legacy_str = dir.display().to_string();
                let modern_str = modern.map_or("$XDG_DATA_HOME/fnm".to_string(), |path| {
                    path.display().to_string()
                });

                outln!(
                    self, Error,
                    "{}\n  It looks like you have the {} directory on your disk.\n  fnm is migrating its default storage location for application data to {}.\n  You can read more about it here: {}\n",
                    "warning:".yellow().bold(),
                    legacy_str.italic(),
                    modern_str.italic(),
                    "https://github.com/schniz/fnm/issues/357".italic()
                );
            }

            return dir;
        }

        modern
            .expect("Can't get data directory")
            .ensure_exists_silently()
    }

    pub fn installations_dir(&self) -> std::path::PathBuf {
        self.base_dir_with_default()
            .join("node-versions")
            .ensure_exists_silently()
    }

    pub fn default_version_dir(&self) -> std::path::PathBuf {
        self.aliases_dir().join("default")
    }

    pub fn aliases_dir(&self) -> std::path::PathBuf {
        self.base_dir_with_default()
            .join("aliases")
            .ensure_exists_silently()
    }

    #[cfg(test)]
    pub fn with_base_dir(mut self, base_dir: Option<std::path::PathBuf>) -> Self {
        self.base_dir = base_dir;
        self
    }
}
