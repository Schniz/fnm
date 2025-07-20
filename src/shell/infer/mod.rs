mod unix;

mod windows;

#[cfg(unix)]
pub use self::unix::infer_shell;
#[cfg(not(unix))]
pub use self::windows::infer_shell;

fn shell_from_string(shell: &str) -> Option<super::Shells> {
    use super::Shells;
    match shell {
        "sh" | "bash" => Some(Shells::Bash),
        "zsh" => Some(Shells::Zsh),
        "fish" => Some(Shells::Fish),
        "pwsh" | "powershell" => Some(Shells::PowerShell),
        #[cfg(windows)]
        "cmd" => Some(Shells::WindowsCmd),
        "nu" => Some(Shells::Nushell),
        cmd_name => {
            log::debug!("binary is not a supported shell: {:?}", cmd_name);
            None
        }
    }
}
