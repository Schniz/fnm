use super::command::Command;
use crate::config::FnmConfig;
use crate::fs::symlink_dir;
use crate::shell::{infer_shell, Shell, AVAILABLE_SHELLS};
use snafu::Snafu;
use std::fmt::Debug;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Default)]
pub struct Env {
    /// The shell syntax to use. Infers when missing.
    #[structopt(long)]
    #[structopt(possible_values = AVAILABLE_SHELLS)]
    shell: Option<Box<dyn Shell>>,
    /// No-op. This is the default now.
    #[structopt(long)]
    multi: bool,
    /// Print the script to change Node versions every directory change
    #[structopt(long)]
    use_on_cd: bool,
}

fn generate_symlink_path(root: &std::path::Path) -> std::path::PathBuf {
    let temp_dir_name = format!(
        "fnm_multishell_{}_{}",
        std::process::id(),
        chrono::Utc::now().timestamp_millis(),
    );
    root.join(temp_dir_name)
}

fn make_symlink(config: &FnmConfig) -> std::path::PathBuf {
    let system_temp_dir = std::env::temp_dir();
    let mut temp_dir = generate_symlink_path(&system_temp_dir);

    while temp_dir.exists() {
        temp_dir = generate_symlink_path(&system_temp_dir);
    }

    symlink_dir(config.default_version_dir(), &temp_dir).expect("Can't create symlink!");
    temp_dir
}

impl Command for Env {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let shell: Box<dyn Shell> = self.shell.unwrap_or_else(&infer_shell);
        let multishell_path = make_symlink(&config);
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
        if self.use_on_cd {
            println!("{}", shell.use_on_cd(&config));
        }
        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum Error {}

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
