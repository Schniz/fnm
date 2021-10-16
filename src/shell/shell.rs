use std::fmt::Debug;
use std::path::Path;

pub trait Shell: Debug {
    fn path(&self, path: &Path) -> String;
    fn set_env_var(&self, name: &str, value: &str) -> String;
    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> String;
    fn rehash(&self) -> Option<String> {
        None
    }
    fn to_structopt_shell(&self) -> structopt::clap::Shell;
}

#[cfg(windows)]
pub const AVAILABLE_SHELLS: &[&str; 5] = &["cmd", "powershell", "bash", "zsh", "fish"];

#[cfg(unix)]
pub const AVAILABLE_SHELLS: &[&str; 4] = &["bash", "zsh", "fish", "powershell"];

impl std::str::FromStr for Box<dyn Shell> {
    type Err = String;

    fn from_str(s: &str) -> Result<Box<dyn Shell>, Self::Err> {
        match s {
            "cmd" => Ok(Box::from(super::windows_cmd::WindowsCmd)),
            "zsh" => Ok(Box::from(super::zsh::Zsh)),
            "bash" => Ok(Box::from(super::bash::Bash)),
            "fish" => Ok(Box::from(super::fish::Fish)),
            "powershell" => Ok(Box::from(super::powershell::PowerShell)),
            shell_type => Err(format!("I don't know the shell type of {:?}", shell_type)),
        }
    }
}

impl From<Box<dyn Shell>> for structopt::clap::Shell {
    fn from(shell: Box<dyn Shell>) -> Self {
        shell.to_structopt_shell()
    }
}
