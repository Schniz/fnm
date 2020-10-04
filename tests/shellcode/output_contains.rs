use super::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub(crate) struct OutputContains<S: Shell, Output: Expression<S>> {
    _s: PhantomData<S>,
    output_of: Output,
    contains: &'static str,
}

impl<S: Shell, Output: Expression<S>> OutputContains<S, Output> {
    pub(crate) fn new(output_of: Output, contains: &'static str) -> Self {
        Self {
            _s: PhantomData,
            output_of,
            contains,
        }
    }
}

impl<E: Expression<Zsh>> Expression<Zsh> for OutputContains<Zsh, E> {
    fn write_shell(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.output_of.write_shell(writer)?;
        write!(writer, " | grep {}", Zsh::shell_escape(self.contains))
    }
}

impl<E: Expression<Bash>> Expression<Bash> for OutputContains<Bash, E> {
    fn write_shell(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.output_of.write_shell(writer)?;
        write!(writer, " | grep {}", Bash::shell_escape(self.contains))
    }
}

impl<E: Expression<Fish>> Expression<Fish> for OutputContains<Fish, E> {
    fn write_shell(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.output_of.write_shell(writer)?;
        write!(writer, " | grep {}", Fish::shell_escape(self.contains))
    }
}

impl<E: Expression<PowerShell>> Expression<PowerShell> for OutputContains<PowerShell, E> {
    fn write_shell(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        // $($__out__ = (fnm ls | findstr 6.11.3); if ($LASTEXITCODE -ne 0) { "WELP" } else { $__out__ })
        write!(writer, "$(")?;
        {
            write!(writer, "$__out__ = $(")?;
            {
                self.output_of.write_shell(writer)?;
                write!(
                    writer,
                    " | Select-String {}",
                    PowerShell::shell_escape(self.contains)
                )?;
            }
            write!(writer, "); ")?;
            write!(writer, "echo $__out__; ")?;
            write!(writer, "if ($__out__ -eq $null)")?;
            write!(writer, "{{ exit 1 }} ")?;
            write!(writer, "else {{ $__out__ }}")?;
        }
        write!(writer, ")")?;
        Ok(())
    }
}
