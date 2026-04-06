use crate::config::FnmConfig;
use crate::version::Version;
use log::debug;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HookError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Hook execution failed: {hook_name} exited with code {}", exit_code.map_or("unknown".to_string(), |code| code.to_string()))]
    HookExecutionFailed {
        hook_name: String,
        exit_code: Option<i32>,
    },
    #[error("Hook not found: {hook_name}")]
    HookNotFound { hook_name: String },
}

/// Represents the different types of hooks that can be executed
#[derive(Debug, Clone)]
pub enum HookType {
    PreInstall,
    PostInstall,
    InstallFailed,
}

impl HookType {
    fn name(&self) -> &'static str {
        match self {
            HookType::PreInstall => "pre-install",
            HookType::PostInstall => "post-install",
            HookType::InstallFailed => "install-failed",
        }
    }
}

/// Context passed to hooks during execution
#[derive(Debug)]
pub struct HookContext {
    /// The Node.js version being operated on
    pub version: String,
    /// The architecture being used
    pub arch: String,
    /// The installation directory for fnm
    pub fnm_dir: PathBuf,
    /// The specific installation directory for this version
    pub installation_dir: PathBuf,
    /// Additional environment variables
    pub env_vars: HashMap<String, String>,
}

impl HookContext {
    pub fn new(version: &Version, config: &FnmConfig) -> Self {
        let version_str = version.v_str();
        let arch = config.arch.to_string();
        let fnm_dir = config.base_dir();
        let installation_dir = config.installations_dir().join(&version_str);

        let mut env_vars = HashMap::new();
        env_vars.insert("FNM_VERSION".to_string(), version_str.clone());
        env_vars.insert("FNM_ARCH".to_string(), arch.clone());
        env_vars.insert("FNM_DIR".to_string(), fnm_dir.to_string_lossy().to_string());
        env_vars.insert(
            "FNM_INSTALLATION_DIR".to_string(),
            installation_dir.to_string_lossy().to_string(),
        );

        Self {
            version: version_str,
            arch,
            fnm_dir,
            installation_dir,
            env_vars,
        }
    }
}

/// Main hooks manager
pub struct HooksManager<'a> {
    config: &'a FnmConfig,
}

impl<'a> HooksManager<'a> {
    pub fn new(config: &'a FnmConfig) -> Self {
        Self { config }
    }

    /// Get the hooks directory path
    fn hooks_dir(&self) -> PathBuf {
        self.config.base_dir().join("hooks")
    }

    /// Check if a hook exists and is executable
    fn hook_exists(&self, hook_type: &HookType) -> Option<PathBuf> {
        let hooks_dir = self.hooks_dir();
        let hook_path = hooks_dir.join(hook_type.name());

        if hook_path.exists() && self.is_executable(&hook_path) {
            Some(hook_path)
        } else {
            None
        }
    }

    /// Check if a file is executable
    #[cfg(unix)]
    fn is_executable(&self, path: &Path) -> bool {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = std::fs::metadata(path) {
            metadata.permissions().mode() & 0o111 != 0
        } else {
            false
        }
    }

    #[cfg(windows)]
    fn is_executable(&self, path: &Path) -> bool {
        use std::ffi::OsStr;
        // On Windows, check for common executable extensions
        if let Some(extension) = path.extension().and_then(OsStr::to_str) {
            matches!(
                extension.to_lowercase().as_str(),
                "exe" | "bat" | "cmd" | "ps1"
            )
        } else {
            // Files without extensions might still be executable (e.g., scripts with shebang)
            true
        }
    }

    /// Execute a hook if it exists
    pub fn execute_hook(
        &self,
        hook_type: HookType,
        context: &HookContext,
    ) -> Result<(), HookError> {
        if let Some(hook_path) = self.hook_exists(&hook_type) {
            debug!("Executing hook: {} for version {}", hook_type.name(), context.version);
            
            let mut command = Command::new(&hook_path);
            
            // Set up environment variables
            for (key, value) in &context.env_vars {
                command.env(key, value);
            }
            
            // Set working directory to fnm directory
            command.current_dir(&context.fnm_dir);
            
            // Configure stdio
            command
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());

            // Execute the hook
            let status = command.status()?;
            
            if status.success() {
                debug!("Hook {} executed successfully", hook_type.name());
                Ok(())
            } else {
                Err(HookError::HookExecutionFailed {
                    hook_name: hook_type.name().to_string(),
                    exit_code: status.code(),
                })
            }
        } else {
            debug!("Hook {} not found or not executable", hook_type.name());
            Ok(())
        }
    }

    /// Execute pre-install hook
    pub fn execute_pre_install(&self, context: &HookContext) -> Result<(), HookError> {
        self.execute_hook(HookType::PreInstall, context)
    }

    /// Execute post-install hook
    pub fn execute_post_install(&self, context: &HookContext) -> Result<(), HookError> {
        self.execute_hook(HookType::PostInstall, context)
    }

    /// Execute install-failed hook
    pub fn execute_install_failed(&self, context: &HookContext) -> Result<(), HookError> {
        self.execute_hook(HookType::InstallFailed, context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_hook_context_creation() {
        let temp_dir = tempdir().unwrap();
        let mut config = crate::config::FnmConfig::default();
        config.base_dir = Some(temp_dir.path().to_path_buf());
        
        let version = Version::parse("18.0.0").unwrap();
        
        let context = HookContext::new(&version, &config);
        
        assert_eq!(context.version, "v18.0.0");
        assert_eq!(context.arch, config.arch.to_string());
        assert_eq!(context.fnm_dir, temp_dir.path());
        assert!(context.env_vars.contains_key("FNM_VERSION"));
        assert!(context.env_vars.contains_key("FNM_ARCH"));
    }

    #[cfg(unix)]
    #[test]
    fn test_hooks_manager() {
        let temp_dir = tempdir().unwrap();
        let hooks_dir = temp_dir.path().join("hooks");
        fs::create_dir_all(&hooks_dir).unwrap();
        
        // Create a mock hook script
        let hook_path = hooks_dir.join("pre-install");
        fs::write(&hook_path, "#!/bin/bash\necho 'Hook executed'").unwrap();
        
        // Make it executable
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook_path, perms).unwrap();
        
        let mut config = crate::config::FnmConfig::default();
        config.base_dir = Some(temp_dir.path().to_path_buf());
        
        let manager = HooksManager::new(&config);
        assert!(manager.hook_exists(&HookType::PreInstall).is_some());
        assert!(manager.hook_exists(&HookType::PostInstall).is_none());
    }
}
