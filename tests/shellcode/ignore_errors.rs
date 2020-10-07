use super::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub(crate) struct IgnoreErrors<S: Shell, E: Expression<S>> {
    _s: PhantomData<S>,
    expr: E,
}

impl<S: Shell, E: Expression<S>> IgnoreErrors<S, E> {
    pub(crate) fn new(expr: E) -> Self {
        Self {
            _s: PhantomData,
            expr,
        }
    }
}

impl<E: Expression<Bash>> Expression<Bash> for IgnoreErrors<Bash, E> {
    fn write_shell(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.expr.write_shell(writer)?;
        Ok(())
    }
}

impl<E: Expression<Zsh>> Expression<Zsh> for IgnoreErrors<Zsh, E> {
    fn write_shell(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.expr.write_shell(writer)?;
        Ok(())
    }
}

impl<E: Expression<Fish>> Expression<Fish> for IgnoreErrors<Fish, E> {
    fn write_shell(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.expr.write_shell(writer)?;
        Ok(())
    }
}

impl<E: Expression<PowerShell>> Expression<PowerShell> for IgnoreErrors<PowerShell, E> {
    fn write_shell(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(writer, r#"$($_tmp_err_action = $ErrorActionPreference;"#)?;
        write!(writer, r#"$ErrorActionPreference = "Continue";"#)?;
        self.expr.write_shell(writer)?;
        write!(writer, ";")?;
        write!(writer, r#"$ErrorActionPreference = $_tmp_err_action)"#)?;
        Ok(())
    }
}
