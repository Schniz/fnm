use std::fmt::{Debug, Display};
use std::path::Path;

use clap::ValueEnum;

pub trait Shell: Debug {
    fn path(
        &self,
        path: &Path,
        order: PathOrder,
        multishell_storage: &Path,
    ) -> anyhow::Result<String>;
    fn set_env_var(&self, name: &str, value: &str) -> String;
    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String>;
    fn rehash(&self) -> Option<&'static str> {
        None
    }
    fn to_clap_shell(&self) -> clap_complete::Shell;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathOrder {
    Prepend,
    Append,
}

pub(crate) fn path_entries(
    path: &Path,
    order: PathOrder,
    multishell_storage: &Path,
) -> Vec<std::path::PathBuf> {
    let current_path = std::env::var_os("PATH").unwrap_or_default();
    let entries: Vec<_> = std::env::split_paths(&current_path).collect();
    ordered_path_entries(entries, path, order, multishell_storage)
}

pub(crate) fn ordered_path_entries(
    mut entries: Vec<std::path::PathBuf>,
    path: &Path,
    order: PathOrder,
    multishell_storage: &Path,
) -> Vec<std::path::PathBuf> {
    match order {
        PathOrder::Prepend => entries.insert(0, path.to_path_buf()),
        PathOrder::Append => {
            entries.retain(|entry| !is_multishell_path(entry, multishell_storage));
            entries.push(path.to_path_buf());
        }
    }

    entries
}

pub(crate) fn quote_powershell(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

pub(crate) fn is_multishell_path(path: &Path, multishell_storage: &Path) -> bool {
    let multishell_path = if cfg!(windows) {
        path
    } else if path.file_name().is_some_and(|name| name == "bin") {
        match path.parent() {
            Some(parent) => parent,
            None => return false,
        }
    } else {
        return false;
    };

    multishell_path
        .file_name()
        .is_some_and(|name| !name.is_empty())
        && multishell_path.parent() == Some(multishell_storage)
}

pub(crate) fn quote_posix(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

pub(crate) fn quote_fish(value: &str) -> String {
    format!("'{}'", value.replace('\\', "\\\\").replace('\'', "\\'"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotes_posix_literals() {
        assert_eq!(quote_posix("space ' $ ` \\ ;"), "'space '\\'' $ ` \\ ;'");
    }

    #[test]
    fn quotes_fish_literals() {
        assert_eq!(quote_fish("space ' $ ` \\ ;"), "'space \\' $ ` \\\\ ;'");
    }

    #[test]
    fn quotes_powershell_literals() {
        assert_eq!(quote_powershell("space ' $ ` & ;"), "'space '' $ ` & ;'");
    }

    #[test]
    fn filters_all_stale_entries_before_appending() {
        let storage = Path::new("storage");
        let stale_one = if cfg!(windows) {
            std::path::PathBuf::from("storage/one")
        } else {
            std::path::PathBuf::from("storage/one/bin")
        };
        let stale_two = if cfg!(windows) {
            std::path::PathBuf::from("storage/two")
        } else {
            std::path::PathBuf::from("storage/two/bin")
        };
        let new_path = if cfg!(windows) {
            std::path::PathBuf::from("storage/new")
        } else {
            std::path::PathBuf::from("storage/new/bin")
        };
        let unrelated = std::path::PathBuf::from("unrelated");

        assert_eq!(
            ordered_path_entries(
                vec![stale_one, unrelated.clone(), stale_two],
                &new_path,
                PathOrder::Append,
                storage,
            ),
            vec![unrelated, new_path]
        );
    }

    #[test]
    fn recognizes_only_direct_multishell_entries() {
        let storage = Path::new("storage");

        if cfg!(windows) {
            assert!(is_multishell_path(Path::new("storage/session"), storage));
            assert!(!is_multishell_path(Path::new("storage"), storage));
            assert!(!is_multishell_path(
                Path::new("storage/session/nested"),
                storage
            ));
            assert!(!is_multishell_path(
                Path::new("storage-prefix/session"),
                storage
            ));
        } else {
            assert!(is_multishell_path(
                Path::new("storage/session/bin"),
                storage
            ));
            assert!(!is_multishell_path(Path::new("storage/bin"), storage));
            assert!(!is_multishell_path(
                Path::new("storage/session/nested/bin"),
                storage
            ));
            assert!(!is_multishell_path(
                Path::new("storage-prefix/session/bin"),
                storage
            ));
        }
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
        }
    }
}

impl From<Box<dyn Shell>> for clap_complete::Shell {
    fn from(shell: Box<dyn Shell>) -> Self {
        shell.to_clap_shell()
    }
}
