mod bash;
mod fish;
mod infer;
mod nushell;
mod powershell;
mod windows_cmd;
mod zsh;

#[allow(clippy::module_inception)]
mod shell;
mod windows_compat;

pub use infer::infer_shell;
pub use shell::{Shell, Shells};
pub use windows_compat::maybe_fix_windows_path;
