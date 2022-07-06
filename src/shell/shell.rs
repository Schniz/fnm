use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;
use anyhow::Result;

pub trait Shell: Debug {
    fn path(&self, path: &Path, formatter: &PathFormatter) -> Result<String>;
    fn set_env_var(&self, name: &str, value: &str) -> String;
    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> Result<String>;
    fn rehash(&self) -> Option<String> {
        None
    }
    fn to_clap_shell(&self) -> clap_complete::Shell;
    fn format_path(&self, path: &Path, formatter: &PathFormatter) -> Result<String>;
}

#[cfg(windows)]
pub const AVAILABLE_SHELLS: &[&str; 5] = &["cmd", "powershell", "bash", "zsh", "fish"];

#[cfg(unix)]
pub const AVAILABLE_SHELLS: &[&str; 4] = &["bash", "zsh", "fish", "powershell"];

impl FromStr for Box<dyn Shell> {
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

impl From<Box<dyn Shell>> for clap_complete::Shell {
    fn from(shell: Box<dyn Shell>) -> Self {
        shell.to_clap_shell()
    }
}

pub const PATH_FORMATTERS: &[&str; 2] = &["cygwin", "wsl"];

#[derive(Debug)]
pub enum PathFormatter {
    Cygwin,
    Wsl,
    None,
}

impl FromStr for PathFormatter {
    type Err = String;
    fn from_str(s: &str) -> Result<PathFormatter, Self::Err> {
        match s {
            "cygwin" => Ok(Self::Cygwin),
            "wsl" => Ok(Self::Wsl),
            "none" => Ok(Self::None),
            formatter => Err(format!("I don't know the Unix path formatter {:?}", formatter)),
        }
    }
}
