use std::borrow::Cow;
use std::fmt::Debug;

pub(crate) trait Shell: Debug {
    fn currently_supported(&self) -> bool;
    fn name(&self) -> &'static str;
    fn binary_name(&self) -> &'static str;
    fn shell_escape(str: &str) -> Cow<str>;
    fn launch_args(&self) -> &'static [&'static str] {
        &[]
    }
}

#[derive(Debug)]
pub(crate) struct Fish;
impl Shell for Fish {
    fn currently_supported(&self) -> bool {
        cfg!(not(windows))
    }
    fn name(&self) -> &'static str {
        "fish"
    }
    fn binary_name(&self) -> &'static str {
        "fish"
    }
    fn shell_escape(str: &str) -> Cow<str> {
        shell_escape::unix::escape(Cow::from(str))
    }
}

#[derive(Debug)]
pub(crate) struct Bash;
impl Shell for Bash {
    fn currently_supported(&self) -> bool {
        true
    }
    fn name(&self) -> &'static str {
        "bash"
    }
    fn binary_name(&self) -> &'static str {
        "bash"
    }
    fn shell_escape(str: &str) -> Cow<str> {
        shell_escape::unix::escape(Cow::from(str))
    }
}

#[derive(Debug)]
pub(crate) struct Zsh;
impl Shell for Zsh {
    fn currently_supported(&self) -> bool {
        cfg!(not(windows))
    }
    fn name(&self) -> &'static str {
        "zsh"
    }
    fn binary_name(&self) -> &'static str {
        "zsh"
    }
    fn shell_escape(str: &str) -> Cow<str> {
        shell_escape::unix::escape(Cow::from(str))
    }
}

#[derive(Debug)]
pub(crate) struct WinCmd;
impl Shell for WinCmd {
    fn currently_supported(&self) -> bool {
        cfg!(windows)
    }
    fn name(&self) -> &'static str {
        "win_cmd"
    }
    fn binary_name(&self) -> &'static str {
        "cmd"
    }
    fn shell_escape(str: &str) -> Cow<str> {
        Cow::from(
            str.replace('\r', "")
                .replace('\n', "^\n\n")
                .replace('%', "%%")
                .replace('|', "^|")
                .replace('(', "^(")
                .replace(')', "^)"),
        )
    }
}

#[derive(Debug)]
pub(crate) struct PowerShell;
impl Shell for PowerShell {
    fn currently_supported(&self) -> bool {
        true
    }
    fn name(&self) -> &'static str {
        "powershell"
    }
    fn binary_name(&self) -> &'static str {
        if cfg!(windows) {
            "powershell"
        } else {
            "pwsh"
        }
    }
    fn shell_escape(str: &str) -> Cow<str> {
        let new_str = format!("'{}'", str.replace('\'', "''"));
        Cow::from(new_str)
    }
    fn launch_args(&self) -> &'static [&'static str] {
        &["-NoProfile"]
    }
}
