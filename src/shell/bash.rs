use crate::version_file_strategy::VersionFileStrategy;

use super::shell::{quote_posix, PathOrder, Shell};
use indoc::formatdoc;
use std::path::Path;

#[derive(Debug)]
pub struct Bash;

impl Shell for Bash {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        clap_complete::Shell::Bash
    }

    fn path(
        &self,
        path: &Path,
        order: PathOrder,
        multishell_storage: &Path,
    ) -> anyhow::Result<String> {
        let path = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't convert path to string"))?;
        let path =
            super::windows_compat::maybe_fix_windows_path(path).unwrap_or_else(|| path.to_string());
        if order == PathOrder::Prepend {
            return Ok(format!("export PATH={path:?}:\"$PATH\""));
        }

        let storage = multishell_storage
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't convert multishell storage path to string"))?;
        let storage = super::windows_compat::maybe_fix_windows_path(storage)
            .unwrap_or_else(|| storage.to_string());
        Ok(append_path_script(&path, &storage))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {name}={value:?}")
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        let version_file_exists_condition = if config.resolve_engines() {
            "-f .node-version || -f .nvmrc || -f package.json"
        } else {
            "-f .node-version || -f .nvmrc"
        };
        let autoload_hook = match config.version_file_strategy() {
            VersionFileStrategy::Local => formatdoc!(
                r"
                    if [[ {version_file_exists_condition} ]]; then
                        fnm use --silent-if-unchanged
                    fi
                ",
                version_file_exists_condition = version_file_exists_condition,
            ),
            VersionFileStrategy::Recursive => String::from(r"fnm use --silent-if-unchanged"),
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
            "#,
            autoload_hook = autoload_hook
        ))
    }
}

pub(super) fn append_path_script(path: &str, multishell_storage: &str) -> String {
    let layout_check = if cfg!(windows) {
        r#"
                case "$__fnm_relative" in
                    ""|*/*) ;;
                    *) continue ;;
                esac"#
    } else {
        r#"
                case "$__fnm_relative" in
                    */bin)
                        __fnm_session=${__fnm_relative%/bin}
                        case "$__fnm_session" in
                            ""|*/*) ;;
                            *) continue ;;
                        esac
                        ;;
                esac"#
    };

    format!(
        r#"export PATH="$({{
    __fnm_remaining="${{PATH-}}:"
    __fnm_has_entry=
    __fnm_new_path=
    while [ -n "$__fnm_remaining" ]; do
        __fnm_entry=${{__fnm_remaining%%:*}}
        __fnm_remaining=${{__fnm_remaining#*:}}
        case "$__fnm_entry" in
            {storage}/*)
                __fnm_relative=${{__fnm_entry#{storage}/}}{layout_check}
                ;;
        esac
        if [ -n "$__fnm_has_entry" ]; then
            __fnm_new_path="$__fnm_new_path:$__fnm_entry"
        else
            __fnm_new_path=$__fnm_entry
            __fnm_has_entry=1
        fi
    done
    if [ -n "$__fnm_has_entry" ]; then
        printf '%s:%s' "$__fnm_new_path" {path}
    else
        printf '%s' {path}
    fi
}})"#,
        storage = quote_posix(multishell_storage),
        path = quote_posix(path),
    )
}
