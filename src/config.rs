use crate::arch::Arch;
use crate::directories::Directories;
use crate::log_level::LogLevel;
use crate::path_ext::PathExt;
use crate::version_file_strategy::VersionFileStrategy;
use url::Url;

#[derive(clap::Parser, Debug)]
pub struct FnmConfig {
    /// <https://nodejs.org/dist/> mirror
    #[clap(
        long,
        env = "FNM_NODE_DIST_MIRROR",
        default_value = "https://nodejs.org/dist",
        global = true,
        hide_env_values = true
    )]
    pub node_dist_mirror: Url,

    /// The root directory of fnm installations.
    #[clap(
        long = "fnm-dir",
        env = "FNM_DIR",
        global = true,
        hide_env_values = true
    )]
    pub base_dir: Option<std::path::PathBuf>,

    /// Where the current node version link is stored.
    /// This value will be populated automatically by evaluating
    /// `fnm env` in your shell profile. Read more about it using `fnm help env`
    #[clap(long, env = "FNM_MULTISHELL_PATH", hide_env_values = true, hide = true)]
    multishell_path: Option<std::path::PathBuf>,

    /// The log level of fnm commands
    #[clap(
        long,
        env = "FNM_LOGLEVEL",
        default_value_t,
        global = true,
        hide_env_values = true
    )]
    log_level: LogLevel,

    /// Override the architecture of the installed Node binary.
    /// Defaults to arch of fnm binary.
    #[clap(
        long,
        env = "FNM_ARCH",
        default_value_t,
        global = true,
        hide_env_values = true,
        hide_default_value = true
    )]
    pub arch: Arch,

    /// A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is
    /// called without a version, or when `--use-on-cd` is configured on evaluation.
    #[clap(
        long,
        env = "FNM_VERSION_FILE_STRATEGY",
        default_value_t,
        global = true,
        hide_env_values = true
    )]
    version_file_strategy: VersionFileStrategy,

    /// Enable corepack support for each new installation.
    /// This will make fnm call `corepack enable` on every Node.js installation.
    /// For more information about corepack see <https://nodejs.org/api/corepack.html>
    #[clap(
        long,
        env = "FNM_COREPACK_ENABLED",
        global = true,
        hide_env_values = true
    )]
    corepack_enabled: bool,

    /// Resolve `engines.node` field in `package.json` whenever a `.node-version` or `.nvmrc` file is not present.
    /// This feature is enabled by default. To disable it, provide `--resolve-engines=false`.
    ///
    /// Note: `engines.node` can be any semver range, with the latest satisfying version being resolved.
    /// Note 2: If you disable it, please open an issue on GitHub describing _why_ you disabled it.
    ///         In the future, disabling it might be a no-op, so it's worth knowing any reason to
    ///         do that.
    #[clap(
        long,
        env = "FNM_RESOLVE_ENGINES",
        global = true,
        hide_env_values = true,
        verbatim_doc_comment
    )]
    #[expect(
        clippy::option_option,
        reason = "clap Option<Option<T>> supports --x and --x=value syntaxes"
    )]
    resolve_engines: Option<Option<bool>>,

    #[clap(skip)]
    directories: Directories,
}

impl Default for FnmConfig {
    fn default() -> Self {
        Self {
            node_dist_mirror: Url::parse("https://nodejs.org/dist/").unwrap(),
            base_dir: None,
            multishell_path: None,
            log_level: LogLevel::Info,
            arch: Arch::default(),
            version_file_strategy: VersionFileStrategy::default(),
            corepack_enabled: false,
            resolve_engines: None,
            directories: Directories::default(),
        }
    }
}

impl FnmConfig {
    pub fn version_file_strategy(&self) -> VersionFileStrategy {
        self.version_file_strategy
    }

    pub fn corepack_enabled(&self) -> bool {
        self.corepack_enabled
    }

    pub fn resolve_engines(&self) -> bool {
        self.resolve_engines.flatten().unwrap_or(true)
    }

    pub fn multishell_path(&self) -> Option<&std::path::Path> {
        match &self.multishell_path {
            None => None,
            Some(v) => Some(v.as_path()),
        }
    }

    pub fn log_level(&self) -> LogLevel {
        self.log_level
    }

    pub fn base_dir_with_default(&self) -> std::path::PathBuf {
        if let Some(dir) = &self.base_dir {
            return dir.clone();
        }

        self.directories.default_base_dir()
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

    pub fn multishell_storage(&self) -> std::path::PathBuf {
        self.directories.multishell_storage()
    }

    #[cfg(test)]
    pub fn with_base_dir(mut self, base_dir: Option<std::path::PathBuf>) -> Self {
        self.base_dir = base_dir;
        self
    }
}
