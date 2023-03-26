use std::fmt::Debug;
use std::path::Path;

pub trait Shell: Debug {
    fn env_init(&mut self, _: &crate::config::FnmConfig) -> anyhow::Result<String> { Ok("".to_string()) }
    fn path(&mut self, path: &Path, config: &crate::config::FnmConfig) -> anyhow::Result<String>;
    fn set_env_var(&mut self, name: &str, value: &str, config: &crate::config::FnmConfig)
        -> String;
    fn use_on_cd(&mut self, config: &crate::config::FnmConfig) -> anyhow::Result<String>;
    fn rehash(&self) -> Option<String> {
        None
    }
    fn to_clap_shell(&self) -> clap_complete::Shell;
}

#[cfg(windows)]
pub const AVAILABLE_SHELLS: &[&str; 6] = &["cmd", "powershell", "bash", "zsh", "fish", "nushell"];

#[cfg(unix)]
pub const AVAILABLE_SHELLS: &[&str; 5] = &["bash", "zsh", "fish", "powershell", "nushell"];

impl std::str::FromStr for Box<dyn Shell> {
    type Err = String;

    fn from_str(s: &str) -> Result<Box<dyn Shell>, Self::Err> {
        match s {
            "cmd" => Ok(Box::from(super::windows_cmd::WindowsCmd)),
            "zsh" => Ok(Box::from(super::zsh::Zsh)),
            "bash" => Ok(Box::from(super::bash::Bash)),
            "fish" => Ok(Box::from(super::fish::Fish)),
            "powershell" => Ok(Box::from(super::powershell::PowerShell)),
            "nushell" => Ok(Box::from(super::nushell::Nushell::default())),
            shell_type => Err(format!("I don't know the shell type of {shell_type:?}",)),
        }
    }
}

impl From<Box<dyn Shell>> for clap_complete::Shell {
    fn from(shell: Box<dyn Shell>) -> Self {
        shell.to_clap_shell()
    }
}
