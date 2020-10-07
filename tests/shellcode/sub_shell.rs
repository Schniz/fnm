use super::expression::Expression;
use super::shell::*;
use std::fmt::Write;
use std::marker::PhantomData;

#[derive(Debug)]
pub(crate) struct SubShell<S: Shell, Exp: Expression<S>> {
    _s: PhantomData<S>,
    expression: Exp,
}

impl<S: Shell, Exp: Expression<S>> SubShell<S, Exp> {
    pub(crate) fn new(expression: Exp) -> Self {
        Self {
            _s: PhantomData,
            expression,
        }
    }
}

impl<E: Expression<Zsh>> Expression<Zsh> for SubShell<Zsh, E> {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        let mut s = String::new();
        self.expression.write_shell(&mut s)?;
        write!(writer, r#"echo {} | zsh"#, Zsh::shell_escape(&s))
    }
}

impl<E: Expression<Bash>> Expression<Bash> for SubShell<Bash, E> {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        let mut s = String::new();
        self.expression.write_shell(&mut s)?;
        write!(writer, r#"echo {} | bash"#, Bash::shell_escape(&s))
    }
}

impl<E: Expression<Fish>> Expression<Fish> for SubShell<Fish, E> {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        let mut s = String::new();
        self.expression.write_shell(&mut s)?;
        write!(writer, r#"fish -c {}"#, Fish::shell_escape(&s))
    }
}

impl<E: Expression<PowerShell>> Expression<PowerShell> for SubShell<PowerShell, E> {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        let mut s = String::new();
        self.expression.write_shell(&mut s)?;
        write!(
            writer,
            r#"echo {} | powershell -NoProfile"#,
            PowerShell::shell_escape(&s)
        )
    }
}
