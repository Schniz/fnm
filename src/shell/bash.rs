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
        Ok(format!("export PATH={:?}:$PATH", path))
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

    fn delete_on_exit(&self, fnm_multishell: &Path) -> Option<String> {
        Some(indoc::formatdoc!(
            r#"
    # appends a command to a trap
    #
    # - 1st arg:  code to add
    # - remaining args:  names of traps to modify
    #
    trap_add() {{
        trap_add_cmd=$1; shift || fatal "${{FUNCNAME}} usage error"
        for trap_add_name in "$@"; do
            trap -- "$(
                # helper fn to get existing trap command from output
                # of trap -p
                extract_trap_cmd() {{ printf '%s\n' "$3"; }}
                # print existing trap command with newline
                eval "extract_trap_cmd $(trap -p "${{trap_add_name}}")"
                # print the new trap command
                printf '%s\n' "${{trap_add_cmd}}"
            )" "${{trap_add_name}}" \
                || fatal "unable to add to trap ${{trap_add_name}}"
        done
    }}
    # set the trace attribute for the above function.  this is
    # required to modify DEBUG or RETURN traps because functions don't
    # inherit them unless the trace attribute is set
    declare -f -t trap_add

    trap_add 'echo hi; rm "{fnm_multishell}"' EXIT
            "#,
            fnm_multishell = fnm_multishell.display()
        ))
    }
}
