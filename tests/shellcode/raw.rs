use super::expression::Expression;
use super::shell::Shell;
use std::fmt::Write;

/// For debugging purposes
#[derive(Debug)]
pub(crate) struct Raw(pub String);

impl<S: Shell> Expression<S> for Raw {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        write!(writer, "{}", self.0)
    }
}
