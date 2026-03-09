use super::command::Command;
use crate::cli::Cli;
use crate::config::FnmConfig;
use clap::CommandFactory;
use clap::Parser;
use clap_mangen::Man;
use thiserror::Error;

#[derive(Parser, Debug)]
pub struct ManPage {
    /// Command path to render (for example: install)
    command: Vec<String>,
}

impl Command for ManPage {
    type Error = Error;

    fn apply(self, _config: &FnmConfig) -> Result<(), Self::Error> {
        let mut app = Cli::command();
        app.build();
        let app = select_command(app, &self.command)?;
        let mut output = std::io::stdout();
        Man::new(app).render(&mut output)?;
        Ok(())
    }
}

fn select_command(mut app: clap::Command, path: &[String]) -> Result<clap::Command, Error> {
    for name in path {
        let command = app
            .get_subcommands()
            .find(|subcommand| subcommand.get_name() == name)
            .cloned()
            .ok_or_else(|| Error::UnknownSubcommand(name.to_owned()))?;
        app = command;
    }

    Ok(app)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown subcommand: {0}")]
    UnknownSubcommand(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
