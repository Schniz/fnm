mod bash;
mod fish;
mod infer;
mod powershell;
mod windows_cmd;
mod zsh;

#[allow(clippy::module_inception)]
mod shell;
mod trap_add_script;

pub use bash::Bash;
pub use fish::Fish;
pub use infer::infer_shell;
pub use powershell::PowerShell;
pub use shell::{Shell, AVAILABLE_SHELLS};
pub use windows_cmd::WindowsCmd;
pub use zsh::Zsh;
