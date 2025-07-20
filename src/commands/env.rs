use super::command::Command;
use crate::config::FnmConfig;
use crate::fs::symlink_dir;
use crate::outln;
use crate::path_ext::PathExt;
use crate::shell::{infer_shell, Shell, Shells};
use clap::ValueEnum;
use colored::Colorize;
use std::collections::HashMap;
use std::fmt::Debug;
use thiserror::Error;

#[allow(clippy::struct_excessive_bools)]
#[derive(clap::Parser, Debug, Default)]
pub struct Env {
    /// The shell syntax to use. Infers when missing.
    #[clap(long)]
    shell: Option<Shells>,
    /// Print JSON instead of shell commands.
    #[clap(long, conflicts_with = "shell")]
    json: bool,
    /// Deprecated. This is the default now.
    #[clap(long, hide = true)]
    multi: bool,
    /// Have a single Node.js version defined globally. This opts out of the "multishell" solution
    /// which allows a Node.js version per shell session. All your shell sessions will share a
    /// global Node.js version, and calling `fnm use` will update the global pointer.
    #[clap(long, conflicts_with = "multi")]
    global: bool,
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

fn make_symlink(config: &FnmConfig) -> Result<std::path::PathBuf, Error> {
    let base_dir = config.multishell_storage().ensure_exists_silently();
    let mut path = base_dir.join(generate_symlink_path());

    while path.exists() {
        path = base_dir.join(generate_symlink_path());
    }

    match symlink_dir(config.default_version_dir(), &path) {
        Ok(()) => Ok(path),
        Err(source) => Err(Error::CantCreateSymlink { source, path }),
    }
}

#[inline]
fn bool_as_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
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

        let multishell_path = if self.global {
            let current_path = config.current_global_version_path();
            crate::fs::two_phase_symlink(&config.default_version_dir(), &current_path).map_err(
                |source| Error::SettingGlobalVersionSymlink {
                    source,
                    path: current_path.clone(),
                },
            )?;
            current_path
        } else {
            make_symlink(config)?
        };

        let base_dir = config.base_dir_with_default();

        let env_vars = [
            ("FNM_MULTISHELL_PATH", multishell_path.to_str().unwrap()),
            (
                "FNM_VERSION_FILE_STRATEGY",
                config.version_file_strategy().as_str(),
            ),
            ("FNM_DIR", base_dir.to_str().unwrap()),
            ("FNM_LOGLEVEL", config.log_level().as_str()),
            ("FNM_NODE_DIST_MIRROR", config.node_dist_mirror.as_str()),
            (
                "FNM_COREPACK_ENABLED",
                bool_as_str(config.corepack_enabled()),
            ),
            ("FNM_RESOLVE_ENGINES", bool_as_str(config.resolve_engines())),
            ("FNM_ARCH", config.arch.as_str()),
        ];

        if self.json {
            println!(
                "{}",
                serde_json::to_string(&HashMap::from(env_vars)).unwrap()
            );
            return Ok(());
        }

        let shell: Box<dyn Shell> = self
            .shell
            .map(Into::into)
            .or_else(infer_shell)
            .ok_or(Error::CantInferShell)?;

        let binary_path = if cfg!(windows) {
            shell.path(&multishell_path)
        } else {
            shell.path(&multishell_path.join("bin"))
        };

        println!("{}", binary_path?);

        for (name, value) in &env_vars {
            println!("{}", shell.set_env_var(name, value));
        }

        if self.use_on_cd {
            println!("{}", shell.use_on_cd(config)?);
        }
        if let Some(v) = shell.rehash() {
            println!("{v}");
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
    #[error("Can't create the symlink for multishells at {path:?}. Maybe there are some issues with permissions for the directory? {source}")]
    CantCreateSymlink {
        #[source]
        source: std::io::Error,
        path: std::path::PathBuf,
    },
    #[error("Can't assign global version symlink at {path:?}. Maybe there are some issues with permissions for the directory? {source}")]
    SettingGlobalVersionSymlink {
        #[source]
        source: crate::fs::TwoPhaseSymlinkError,
        path: std::path::PathBuf,
    },
    #[error(transparent)]
    ShellError {
        #[from]
        source: anyhow::Error,
    },
}

fn shells_as_string() -> String {
    Shells::value_variants()
        .iter()
        .map(|x| format!("* {x}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke() {
        let config = FnmConfig::default();
        Env {
            #[cfg(windows)]
            shell: Some(Shells::Cmd),
            #[cfg(not(windows))]
            shell: Some(Shells::Bash),
            ..Default::default()
        }
        .call(config);
    }
}
