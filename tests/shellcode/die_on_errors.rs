use super::expression::Expression;
use super::shell::{Bash, Fish, PowerShell, WinCmd, Zsh};
use indoc::writedoc;
use std::fmt::Write;

#[derive(Debug)]
pub(crate) struct DieOnErrors;

impl Expression<Bash> for DieOnErrors {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        writedoc!(
            writer,
            r#"
                set -e
                shopt -s expand_aliases
            "#
        )
    }
}

impl Expression<Zsh> for DieOnErrors {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        write!(writer, "set -e")
    }
}

impl Expression<Fish> for DieOnErrors {
    fn write_shell(&self, _writer: &mut impl Write) -> std::fmt::Result {
        Ok(())
    }
}

impl Expression<PowerShell> for DieOnErrors {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        write!(writer, r#"$ErrorActionPreference = "Stop""#)
    }
}

impl Expression<WinCmd> for DieOnErrors {
    fn write_shell(&self, _writer: &mut impl Write) -> std::fmt::Result {
        Ok(())
    }
}
