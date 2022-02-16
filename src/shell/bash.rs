use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use indoc::{formatdoc, indoc};
use std::path::Path;

#[derive(Debug)]
pub struct Bash;

impl Shell for Bash {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        clap_complete::Shell::Bash
    }

    fn path(&self, path: &Path) -> anyhow::Result<String> {
        let path = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't convert path to string"))?;
        let multishell_storage = crate::directories::multishell_storage();
        Ok(formatdoc!(
            r#"
                new_path="{path}"
                IFS=":"
                for p in $PATH; do
                    if ! echo $p | grep -q {multishell_storage}; then
                        new_path="$new_path:$p"
                    fi
                done
                unset IFS
                export PATH=$new_path
                unset new_path
            "#,
            multishell_storage = multishell_storage.display(),
            path = path,
        ))
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
}
