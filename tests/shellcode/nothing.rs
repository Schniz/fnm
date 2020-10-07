use super::expression::Expression;
use super::shell::Shell;
use std::fmt::Write;
use std::marker::PhantomData;

#[derive(Debug)]
pub(crate) struct Nothing<S: Shell> {
    shell: PhantomData<S>,
}

pub(crate) fn empty_shell_script<S: Shell>(_s: &S) -> Nothing<S> {
    Nothing { shell: PhantomData }
}

impl<S: Shell> Expression<S> for Nothing<S> {
    fn write_shell(&self, _writer: &mut impl Write) -> std::fmt::Result {
        Ok(())
    }
}
