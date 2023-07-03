mod unix;

mod windows;

#[cfg(unix)]
pub use self::unix::infer_shell;
#[cfg(not(unix))]
pub use self::windows::infer_shell;

use super::Shells;

pub(self) fn shell_from_string(shell: &str) -> Option<Shells> {
    match shell {
        "sh" | "bash" => return Some(Shells::Bash),
        "zsh" => return Some(Shells::Zsh),
        "fish" => return Some(Shells::Fish),
        "pwsh" | "powershell" => return Some(Shells::PowerShell),
        #[cfg(windows)]
        "cmd" => return Some(Shells::Cmd),
        cmd_name => log::debug!("binary is not a supported shell: {:?}", cmd_name),
    };
    None
}
