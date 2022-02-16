use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use indoc::{formatdoc, indoc};
use std::path::Path;

#[derive(Debug)]
pub struct Zsh;

impl Shell for Zsh {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        clap_complete::Shell::Zsh
    }

    fn path(&self, path: &Path) -> anyhow::Result<String> {
        let path = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Path is not valid UTF-8"))?;
        let multishell_storage = crate::directories::multishell_storage();
        Ok(formatdoc!(
            r#"
                new_path="{path}"
                for p in ${{(s.:.)PATH}}; do
                    if ! echo $p | grep -q {multishell_storage}; then
                        new_path="$new_path:$p"
                    fi
                done
                export PATH=$new_path
                unset new_path
            "#,
            multishell_storage = multishell_storage.display(),
            path = path
        ))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {}={:?}", name, value)
    }

    fn rehash(&self) -> Option<String> {
        Some("rehash".to_string())
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
                autoload -U add-zsh-hook
                _fnm_autoload_hook () {{
                    {autoload_hook}
                }}

                add-zsh-hook chpwd _fnm_autoload_hook \
                    && _fnm_autoload_hook
            "#,
            autoload_hook = autoload_hook
        ))
    }
}
