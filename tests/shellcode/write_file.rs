use super::*;

#[derive(Debug)]
pub struct WriteFile {
    name: &'static str,
    contents: &'static str,
}

impl WriteFile {
    pub(crate) fn new(name: &'static str, contents: &'static str) -> Self {
        Self { name, contents }
    }
}

impl<S: Shell> Expression<S> for WriteFile {
    fn write_shell(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(
            writer,
            "echo {} > {}",
            S::shell_escape(self.contents),
            S::shell_escape(self.name)
        )
    }
}
