use crate::config::FnmConfig;
use crate::outln;
use colored::Colorize;

pub trait Command: Sized {
    type Error: miette::Diagnostic + std::marker::Send + std::marker::Sync + 'static;
    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error>;

    fn handle_error(err: Self::Error, config: &FnmConfig) {
        let err = miette::Report::from(err);
        let err_s = format!("{err:?}");
        outln!(config, Error, "{} {}", "error:".red().bold(), err_s.red());
        std::process::exit(1);
    }

    fn call(self, config: FnmConfig) {
        match self.apply(&config) {
            Ok(()) => (),
            Err(err) => Self::handle_error(err, &config),
        }
    }
}
