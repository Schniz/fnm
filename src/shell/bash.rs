use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use anyhow::Result;
use indoc::{formatdoc, indoc};
use std::path::Path;
use crate::shell::PathFormatter;
use crate::unixpath::to_unix_path;

#[derive(Debug)]
pub struct Bash;

impl Shell for Bash {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        clap_complete::Shell::Bash
    }

    fn path(&self, path: &Path, formatter: &PathFormatter) -> anyhow::Result<String> {
        Ok(format!("export PATH={}:$PATH", &self.format_path(path, formatter)?))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {}={:?}", name, value)
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        let autoload_hook = match config.version_file_strategy() {
            VersionFileStrategy::Local => indoc!(
                r#"
                    if [[ -f .node-version || -f .nvmrc ]]; then
                        fnm use --silent-if-unchanged
                    fi
                "#
            ),
            VersionFileStrategy::Recursive => r#"fnm use --silent-if-unchanged"#,
        };
        Ok(formatdoc!(
            r#"
                __fnm_use_if_file_found() {{
                    {autoload_hook}
                }}

                __fnmcd() {{
                    \cd "$@" || return $?
                    __fnm_use_if_file_found
                }}

                alias cd=__fnmcd
                __fnm_use_if_file_found
            "#,
            autoload_hook = autoload_hook
        ))
    }

    fn format_path(&self, path: &Path, formatter: &PathFormatter) -> Result<String> {
        to_unix_path(path, formatter)
    }
}
