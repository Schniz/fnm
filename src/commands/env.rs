use super::command::Command;
use crate::config::FnmConfig;
use crate::fs::symlink_dir;
use crate::outln;
use crate::path_ext::PathExt;
use crate::shell::{infer_shell, Shell, AVAILABLE_SHELLS};
use colored::Colorize;
use std::fmt::Debug;
use std::path::PathBuf;
use thiserror::Error;

#[derive(clap::Parser, Debug, Default)]
pub struct Env {
    /// The shell syntax to use. Infers when missing.
    #[clap(long)]
    #[clap(possible_values = AVAILABLE_SHELLS)]
    shell: Option<Box<dyn Shell>>,
    /// Deprecated. This is the default now.
    #[clap(long, hide = true)]
    multi: bool,
    /// Print the script to change Node versions every directory change
    #[clap(long)]
    use_on_cd: bool,
}

fn generate_symlink_path() -> String {
    format!(
        "{}_{}",
        std::process::id(),
        chrono::Utc::now().timestamp_millis(),
    )
}

fn symlink_base_dir() -> PathBuf {
    match dirs::cache_dir() {
        Some(cache_dir) => cache_dir.join("fnm").join("multishells"),
        None => std::env::temp_dir().join("fnm_multishells"),
    }
    .ensure_exists_silently()
}

fn make_symlink(config: &FnmConfig) -> std::path::PathBuf {
    let base_dir = symlink_base_dir();
    let mut temp_dir = base_dir.join(generate_symlink_path());

    while temp_dir.exists() {
        temp_dir = base_dir.join(generate_symlink_path());
    }

    symlink_dir(config.default_version_dir(), &temp_dir).expect("Can't create symlink!");
    temp_dir
}

impl Command for Env {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        if self.multi {
            outln!(
                config,
                Error,
                "{} {} is deprecated. This is now the default.",
                "warning:".yellow().bold(),
                "--multi".italic()
            );
        }

        let shell: Box<dyn Shell> = self
            .shell
            .or_else(&infer_shell)
            .ok_or(Error::CantInferShell)?;
        let multishell_path = make_symlink(config);
        let binary_path = if cfg!(windows) {
            multishell_path.clone()
        } else {
            multishell_path.join("bin")
        };
        println!("{}", shell.path(&binary_path));
        println!(
            "{}",
            shell.set_env_var("FNM_MULTISHELL_PATH", multishell_path.to_str().unwrap())
        );
        println!(
            "{}",
            shell.set_env_var(
                "FNM_VERSION_FILE_STRATEGY",
                config.version_file_strategy().as_str()
            )
        );
        println!(
            "{}",
            shell.set_env_var("FNM_DIR", config.base_dir_with_default().to_str().unwrap())
        );
        println!(
            "{}",
            shell.set_env_var("FNM_LOGLEVEL", config.log_level().clone().into())
        );
        println!(
            "{}",
            shell.set_env_var("FNM_NODE_DIST_MIRROR", config.node_dist_mirror.as_str())
        );
        println!(
            "{}",
            shell.set_env_var("FNM_ARCH", &config.arch.to_string())
        );
        if self.use_on_cd {
            println!("{}", shell.use_on_cd(config));
        }
        if let Some(v) = shell.rehash() {
            println!("{}", v);
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "{}\n{}\n{}\n{}",
        "Can't infer shell!",
        "fnm can't infer your shell based on the process tree.",
        "Maybe it is unsupported? we support the following shells:",
        shells_as_string()
    )]
    CantInferShell,
}

fn shells_as_string() -> String {
    AVAILABLE_SHELLS
        .iter()
        .map(|x| format!("* {}", x))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke() {
        use crate::shell;
        let config = FnmConfig::default();
        let shell: Box<dyn Shell> = if cfg!(windows) {
            Box::from(shell::WindowsCmd)
        } else {
            Box::from(shell::Bash)
        };
        Env {
            shell: Some(shell),
            ..Env::default()
        }
        .call(config);
    }
}
