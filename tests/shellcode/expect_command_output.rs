use super::expression::Expression;
use super::shell::{Bash, Fish, PowerShell, Shell, WinCmd, Zsh};
use indoc::writedoc;
use std::fmt::Write;

#[derive(Debug)]
pub(crate) struct ExpectCommandOutput<S: Shell, Command: Expression<S>> {
    _shell: std::marker::PhantomData<S>,
    command: Command,
    expected_output: &'static str,
    message: &'static str,
}

impl<S: Shell, Command: Expression<S>> ExpectCommandOutput<S, Command> {
    pub(crate) fn new(
        command: Command,
        expected_output: &'static str,
        message: &'static str,
    ) -> Self {
        Self {
            _shell: std::marker::PhantomData,
            command,
            expected_output,
            message,
        }
    }
}

impl<E: Expression<Bash>> Expression<Bash> for ExpectCommandOutput<Bash, E> {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        let mut command = String::new();
        self.command.write_shell(&mut command)?;

        writedoc!(
            writer,
            r#"
                if [ "$({command})" != "{expected_output}" ]; then
                    echo 'Expected {message} to be {expected_output:?}, Got: '"$({command})"
                    exit 1
                fi
            "#,
            command = command,
            message = self.message,
            expected_output = self.expected_output,
        )
    }
}

impl<E: Expression<Zsh>> Expression<Zsh> for ExpectCommandOutput<Zsh, E> {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        let mut command = String::new();
        self.command.write_shell(&mut command)?;

        writedoc!(
            writer,
            r#"
                if [ "$({command})" != "{expected_output}" ]; then
                    echo 'Expected {message} to be {expected_output:?}, Got: '"$({command})"
                    exit 1
                fi
            "#,
            command = command,
            message = self.message,
            expected_output = self.expected_output,
        )
    }
}

impl<E: Expression<Fish>> Expression<Fish> for ExpectCommandOutput<Fish, E> {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        let mut command = String::new();
        self.command.write_shell(&mut command)?;

        writedoc!(
            writer,
            r#"
                if test ({command}) != "{expected_output}"
                    echo 'Expected {message} to be {expected_output:?}, Got: '({command})
                    exit 1
                end
            "#,
            command = command,
            expected_output = self.expected_output,
            message = self.message,
        )
    }
}

impl<E: Expression<PowerShell>> Expression<PowerShell> for ExpectCommandOutput<PowerShell, E> {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        let mut command = String::new();
        self.command.write_shell(&mut command)?;

        writedoc!(
            writer,
            r#"
                If ("$({command})" -ne "{expected_output}") {{
                    Write-Output ('Expected {message} to be {expected_output:?}, Got: ' + $({command}))
                    exit 1
                }}
            "#,
            command = command,
            expected_output = self.expected_output,
            message = self.message,
        )
    }
}

impl<E: Expression<WinCmd>> Expression<WinCmd> for ExpectCommandOutput<WinCmd, E> {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        let mut command = String::new();
        self.command.write_shell(&mut command)?;

        writedoc!(
            writer,
            r#"
                {command} | findstr {expected_output}
                if %errorlevel% neq 0 (
                    echo {message} does not match {expected_output:?}
                    exit 1
                )
            "#,
            command = command,
            expected_output = WinCmd::shell_escape(self.expected_output),
            message = self.message,
        )
    }
}
