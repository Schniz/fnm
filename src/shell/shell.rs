use std::fmt::{Debug, Display};
use std::path::Path;

use clap::ValueEnum;

pub trait Shell: Debug {
    fn path(&self, path: &Path) -> anyhow::Result<String>;
    fn set_env_var(&self, name: &str, value: &str) -> String;
    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String>;
    fn rehash(&self) -> Option<String> {
        None
    }
    fn to_clap_shell(&self) -> clap_complete::Shell;
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Shells {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    #[cfg(windows)]
    Cmd,
}

impl Display for Shells {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shells::Bash => write!(f, "bash"),
            Shells::Zsh => write!(f, "zsh"),
            Shells::Fish => write!(f, "fish"),
            Shells::PowerShell => write!(f, "powershell"),
            #[cfg(windows)]
            Shells::Cmd => write!(f, "cmd"),
        }
    }
}

impl Shells {
    fn actual_shell(&self) -> Box<dyn Shell> {
        match self {
            Shells::Zsh => Box::from(super::zsh::Zsh),
            Shells::Bash => Box::from(super::bash::Bash),
            Shells::Fish => Box::from(super::fish::Fish),
            Shells::PowerShell => Box::from(super::powershell::PowerShell),
            #[cfg(windows)]
            Shells::Cmd => Box::from(super::windows_cmd::WindowsCmd),
        }
    }
}

impl Shell for Shells {
    fn path(&self, path: &Path) -> anyhow::Result<String> {
        self.actual_shell().path(path)
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        self.actual_shell().set_env_var(name, value)
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        self.actual_shell().use_on_cd(config)
    }

    fn to_clap_shell(&self) -> clap_complete::Shell {
        self.actual_shell().to_clap_shell()
    }
}

impl From<Shells> for clap_complete::Shell {
    fn from(shell: Shells) -> Self {
        shell.to_clap_shell()
    }
}
