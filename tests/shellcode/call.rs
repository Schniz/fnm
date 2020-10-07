use super::expression::Expression;
use super::shell::Shell;
use std::fmt::Write;

#[derive(Debug)]
pub(crate) struct Call {
    binary: Box<&'static str>,
    args: Vec<&'static str>,
}

impl Call {
    pub(crate) fn new(binary: &'static str, args: Vec<&'static str>) -> Self {
        Self {
            binary: Box::from(binary),
            args,
        }
    }
}

impl<S: Shell> Expression<S> for Call {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        write!(writer, "{}", self.binary)?;
        for arg in &self.args {
            write!(writer, " {}", arg)?;
        }
        Ok(())
    }
}
