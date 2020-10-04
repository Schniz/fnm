use super::line_separated_expressions::LineSeparatedExpressions;
use super::shell::Shell;
use std::fmt::{Debug, Write};

pub(crate) trait Expression<E: Shell>: Debug + Sized {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result;

    fn then<B: Expression<E>>(self, other: B) -> LineSeparatedExpressions<E, Self, B> {
        LineSeparatedExpressions {
            _shell: std::marker::PhantomData,
            a: self,
            b: other,
        }
    }
}

impl<S: Shell> Expression<S> for () {
    fn write_shell(&self, _writer: &mut impl Write) -> std::fmt::Result {
        Ok(())
    }
}
