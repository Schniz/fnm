use super::command::Command;
use crate::config::FnmConfig;
use crate::shell::infer_shell;
use crate::{cli::Cli, shell::Shells};
use clap::{Args, CommandFactory, ValueEnum};
use clap_complete::Generator;
use thiserror::Error;

#[derive(Args, Debug)]
pub struct Completions {
    /// The shell syntax to use. Infers when missing.
    #[arg(long)]
    shell: Option<Shells>,
}

impl Command for Completions {
    type Error = Error;

    fn apply(self, _config: &FnmConfig) -> Result<(), Self::Error> {
        let mut stdio = std::io::stdout();
        let shell: Shells = self
            .shell
            .or_else(infer_shell)
            .ok_or(Error::CantInferShell)?;
        let mut app = Cli::command();
        app.build();
        shell.generate(&app, &mut stdio);
        Ok(())
    }
}

#[derive(Error, Debug)]
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
    #[cfg(not(windows))]
    fn test_smoke() {
        let config = FnmConfig::default();
        Completions {
            shell: Some(Shells::Bash),
        }
        .call(config);
    }
}
