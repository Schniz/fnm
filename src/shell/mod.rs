mod bash;
mod fish;
mod infer;
mod powershell;
mod windows_cmd;
mod zsh;

#[allow(clippy::module_inception)]
mod shell;

pub use bash::Bash;
pub use fish::Fish;
pub use powershell::PowerShell;
pub use shell::{Shell, AVAILABLE_SHELLS};
pub use windows_cmd::WindowsCmd;
pub use zsh::Zsh;

#[cfg(windows)]
pub fn infer_shell() -> Option<Box<dyn Shell>> {
    self::infer::windows::infer_shell()
}

#[cfg(unix)]
pub fn infer_shell() -> Option<Box<dyn Shell>> {
    infer::unix::infer_shell()
}
