use super::expression::Expression;
use super::shell::Shell;
use std::fmt::Write;
use std::marker::PhantomData;

#[derive(Debug)]
pub(crate) struct LineSeparatedExpressions<S: Shell, A: Expression<S>, B: Expression<S>> {
    pub(crate) _shell: std::marker::PhantomData<S>,
    pub(crate) a: A,
    pub(crate) b: B,
}

impl<S: Shell, A: Expression<S>, B: Expression<S>> LineSeparatedExpressions<S, A, B> {
    pub fn then<C: Expression<S>>(
        self,
        c: C,
    ) -> LineSeparatedExpressions<S, LineSeparatedExpressions<S, A, B>, C> {
        LineSeparatedExpressions {
            _shell: PhantomData,
            a: self,
            b: c,
        }
    }
}

impl<S: Shell, B: Expression<S>> LineSeparatedExpressions<S, (), B> {
    pub(crate) fn new(b: B) -> Self {
        Self {
            _shell: std::marker::PhantomData,
            a: (),
            b,
        }
    }
}

impl<S: Shell, A: Expression<S>, B: Expression<S>> Expression<S>
    for LineSeparatedExpressions<S, A, B>
{
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        self.a.write_shell(writer)?;
        write!(writer, "\n")?;
        self.b.write_shell(writer)
    }
}

mod tests {
    use super::super::raw::Raw;
    use super::super::shell::Zsh;
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_line_separated_expression() {
        let mut s = String::new();
        LineSeparatedExpressions::<Zsh, _, _>::new(Raw("Hello".into()))
            .then(Raw("World".into()))
            .then(Raw("Other".into()))
            .write_shell(&mut s)
            .unwrap();
        assert_eq!(s.trim(), "Hello\nWorld\nOther");
    }
}
