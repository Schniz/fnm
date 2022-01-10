use super::command::Command;
use crate::cli::Cli;
use crate::config::FnmConfig;
use crate::shell::{infer_shell, AVAILABLE_SHELLS};
use structopt::clap::Shell;
use structopt::StructOpt;
use thiserror::Error;

#[derive(StructOpt, Debug)]
pub struct Completions {
    /// The shell syntax to use. Infers when missing.
    #[structopt(long, possible_values = &Shell::variants())]
    shell: Option<Shell>,
}

impl Command for Completions {
    type Error = Error;

    fn apply(self, _config: &FnmConfig) -> Result<(), Self::Error> {
        use Error::*;
        let mut stdio = std::io::stdout();
        let shell = self
            .shell
            .or_else(|| infer_shell().map(Into::into))
            .ok_or(CantInferShell)?;
        Cli::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut stdio);
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
    AVAILABLE_SHELLS
        .iter()
        .map(|x| format!("* {}", x))
        .collect::<Vec<_>>()
        .join("\n")
}
