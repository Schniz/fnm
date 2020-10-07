use super::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub(crate) struct GetStderr<S: Shell, E: Expression<S>> {
    _s: PhantomData<S>,
    expr: E,
}

impl<S: Shell, E: Expression<S>> GetStderr<S, E> {
    pub(crate) fn new(expr: E) -> Self {
        Self {
            _s: PhantomData,
            expr,
        }
    }
}

impl<S: Shell, E: Expression<S>> Expression<S> for GetStderr<S, E> {
    fn write_shell(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.expr.write_shell(writer)?;
        write!(writer, " 2>&1")?;
        Ok(())
    }
}
