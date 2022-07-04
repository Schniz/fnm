use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use anyhow::{Context, Result};
use indoc::{formatdoc, indoc};
use std::path::Path;
use crate::cygpath::cygpath;

#[derive(Debug)]
pub struct Fish;

impl Shell for Fish {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        clap_complete::Shell::Fish
    }

    fn path(&self, path: &Path) -> anyhow::Result<String> {
        let path = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't convert path to string"))?;
        Ok(format!("set -gx PATH {} $PATH;", &self.format_path(path)?))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("set -gx {name} {value:?};", name = name, value = value)
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
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

    fn format_path(&self, path: &str) -> Result<String> {
        if cfg!(windows) {
            cygpath(path).context("Failed to convert Windows path to Unix")
        } else {
            Ok(path.to_string())
        }
    }
}
