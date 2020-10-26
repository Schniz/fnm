use std::fmt::Debug;
use std::path::PathBuf;

pub trait Shell: Debug {
    fn path(&self, path: &PathBuf) -> String;
    fn set_env_var(&self, name: &str, value: &str) -> String;
    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> String;
    fn into_structopt_shell(&self) -> structopt::clap::Shell;
    fn echo_found(&self, _file_name: &str, _config: &crate::config::FnmConfig) -> Option<String> {
        None
    }
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

impl Into<structopt::clap::Shell> for Box<dyn Shell> {
    fn into(self) -> structopt::clap::Shell {
        self.into_structopt_shell()
    }
}
