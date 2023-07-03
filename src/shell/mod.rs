mod bash;
mod fish;
mod infer;
mod powershell;
mod windows_cmd;
mod zsh;

#[allow(clippy::module_inception)]
mod shell;
mod windows_compat;

pub use bash::Bash;
pub use fish::Fish;
pub use infer::infer_shell;
pub use powershell::PowerShell;
pub use shell::{Shell, Shells};
pub use windows_cmd::WindowsCmd;
pub use windows_compat::maybe_fix_windows_path;
pub use zsh::Zsh;
