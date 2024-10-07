use std::fmt::{Debug, Display};
use std::path::Path;

use clap::ValueEnum;

pub trait Shell: Debug {
    fn path(&self, path: &Path) -> anyhow::Result<String>;
    fn set_env_var(&self, name: &str, value: &str) -> String;
    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String>;
    fn rehash(&self) -> Option<&'static str> {
        None
    }
    fn to_clap_shell(&self) -> clap_complete::Shell;
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Shells {
    Bash,
    Zsh,
    Fish,
    #[clap(name = "powershell", alias = "power-shell")]
    PowerShell,
    #[cfg(windows)]
    Cmd,
}

impl Display for Shells {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shells::Bash => f.write_str("bash"),
            Shells::Zsh => f.write_str("zsh"),
            Shells::Fish => f.write_str("fish"),
            Shells::PowerShell => f.write_str("powershell"),
            #[cfg(windows)]
            Shells::Cmd => f.write_str("cmd"),
        }
    }
}

impl From<Shells> for Box<dyn Shell> {
    fn from(shell: Shells) -> Box<dyn Shell> {
        match shell {
            Shells::Zsh => Box::from(super::zsh::Zsh),
            Shells::Bash => Box::from(super::bash::Bash),
            Shells::Fish => Box::from(super::fish::Fish),
            Shells::PowerShell => Box::from(super::powershell::PowerShell),
            #[cfg(windows)]
            Shells::Cmd => Box::from(super::windows_cmd::WindowsCmd),
        }
    }
}

impl From<Box<dyn Shell>> for clap_complete::Shell {
    fn from(shell: Box<dyn Shell>) -> Self {
        shell.to_clap_shell()
    }
}
