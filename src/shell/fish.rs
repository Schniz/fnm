use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use anyhow::Result;
use indoc::{formatdoc, indoc};
use std::path::Path;
use crate::shell::PathFormatter;
use crate::unixpath::to_unix_path;

#[derive(Debug)]
pub struct Fish;

impl Shell for Fish {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        clap_complete::Shell::Fish
    }

    fn path(&self, path: &Path, formatter: &PathFormatter) -> Result<String> {
        Ok(format!("set -gx PATH {} $PATH;", &self.format_path(path, formatter)?))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("set -gx {name} {value:?};", name = name, value = value)
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> Result<String> {
        let autoload_hook = match config.version_file_strategy() {
            VersionFileStrategy::Local => indoc!(
                r#"
                    if test -f .node-version -o -f .nvmrc
                        fnm use --silent-if-unchanged
                    end
                "#
            ),
            VersionFileStrategy::Recursive => r#"fnm use --silent-if-unchanged"#,
        };
        Ok(formatdoc!(
            r#"
                function _fnm_autoload_hook --on-variable PWD --description 'Change Node version on directory change'
                    status --is-command-substitution; and return
                    {autoload_hook}
                end

                _fnm_autoload_hook
            "#,
            autoload_hook = autoload_hook
        ))
    }

    fn format_path(&self, path: &Path, formatter: &PathFormatter) -> Result<String> {
        to_unix_path(path, formatter)
    }
}
