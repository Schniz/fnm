use std::fmt::{Debug, Display};
use std::path::Path;

use clap::ValueEnum;
use clap_complete::Generator;

pub trait Shell: Debug {
    fn path(&self, path: &Path) -> anyhow::Result<String>;
    fn set_env_var(&self, name: &str, value: &str) -> String;
    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String>;
    fn rehash(&self) -> Option<&'static str> {
        None
    }
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
    Nushell,
}

impl Generator for Shells {
    fn file_name(&self, name: &str) -> String {
        match self {
            Self::Bash => clap_complete::Shell::Bash.file_name(name),
            Self::Zsh => clap_complete::Shell::Zsh.file_name(name),
            Self::Fish => clap_complete::Shell::Fish.file_name(name),
            Self::PowerShell => clap_complete::Shell::PowerShell.file_name(name),
            #[cfg(windows)]
            Self::Cmd => panic!("Shell completion is not supported for Windows Command Prompt. Maybe try using PowerShell for a better experience?"),
            Self::Nushell => clap_complete_nushell::Nushell.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn std::io::Write) {
        match self {
            Self::Bash => clap_complete::Shell::Bash.generate(cmd, buf),
            Self::Zsh => clap_complete::Shell::Zsh.generate(cmd, buf),
            Self::Fish => clap_complete::Shell::Fish.generate(cmd, buf),
            Self::PowerShell => clap_complete::Shell::PowerShell.generate(cmd, buf),
            #[cfg(windows)]
            Self::Cmd => panic!("Shell completion is not supported for Windows Command Prompt. Maybe try using PowerShell for a better experience?"),
            Self::Nushell => clap_complete_nushell::Nushell.generate(cmd, buf),
        }
    }
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
            Shells::Nushell => f.write_str("nushell"),
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
            Shells::Nushell => Box::from(super::nushell::Nushell),
        }
    }
}
