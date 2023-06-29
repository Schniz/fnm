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
        Ok(format!("export PATH={path:?}:$PATH"))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {name}={value:?}")
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

    fn delete_on_exit(&self, fnm_multishell: &Path) -> Option<String> {
        Some(indoc::formatdoc!(
            r#"
    {TRAP_ADD}

    __fnm_trap_add__ 'rm "{fnm_multishell}"' EXIT
            "#,
            fnm_multishell = fnm_multishell.display(),
            TRAP_ADD = TRAP_ADD
        ))
    }
}

/// This code is based on [Richard Hansen's answer on `StackOverflow`](https://stackoverflow.com/a/7287873/1176984)
///
/// Usage:
/// ```bash
/// __fnm_trap_add__ 'echo "hello"' EXIT
/// ```
pub const TRAP_ADD: &str = indoc::indoc!(
    r#"
    __fnm_trap_add__() {
        __fnm_trap_add___cmd=$1; shift || fatal "${FUNCNAME} usage error"
        for __fnm_trap_add___name in "$@"; do
            trap -- "$(
                extract_trap_cmd() { printf '%s\n' "$3"; }
                eval "extract_trap_cmd $(trap -p "${__fnm_trap_add___name}")"
                printf '%s\n' "${__fnm_trap_add___cmd}"
            )" "${__fnm_trap_add___name}" \
                || fatal "unable to add to trap ${__fnm_trap_add___name}"
        done
    }
    declare -f -t __fnm_trap_add__
    "#
);
