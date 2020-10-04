use super::command::Command;
use crate::cli::Cli;
use crate::config::FnmConfig;
use crate::shell::infer_shell;
use structopt::clap::Shell;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Completions {
    /// The shell syntax to use. Infers when missing.
    #[structopt(long, possible_values = &Shell::variants())]
    shell: Option<Shell>,
}

impl Command for Completions {
    type Error = Error;

    fn apply(self, _config: &FnmConfig) -> Result<(), Self::Error> {
        let mut stdio = std::io::stdout();
        let shell = self.shell.unwrap_or_else(|| infer_shell().into());
        Cli::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut stdio);
        Ok(())
    }
}

#[derive(snafu::Snafu, Debug)]
pub enum Error {}
