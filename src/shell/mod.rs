mod bash;
mod fish;
mod infer;
mod powershell;
mod shell;
mod windows_cmd;
mod zsh;

pub use bash::Bash;
pub use fish::Fish;
pub use powershell::PowerShell;
pub use shell::{Shell, AVAILABLE_SHELLS};
pub use windows_cmd::WindowsCmd;
pub use zsh::Zsh;

/// Always returns WindowsCmd (because this is what we support on Windows)
#[cfg(windows)]
pub fn infer_shell() -> Box<dyn Shell> {
    let inferred = self::infer::windows::infer_shell();
    inferred.expect("Can't infer shell")
}

/// Tries to infer shell or dies trying
#[cfg(unix)]
pub fn infer_shell() -> Box<dyn Shell> {
    infer::unix::infer_shell().expect("Can't infer shell")
}
