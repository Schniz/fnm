use crate::version_file_strategy::VersionFileStrategy;

use super::shell::{quote_fish, PathOrder, Shell};
use indoc::formatdoc;
use std::path::Path;

#[derive(Debug)]
pub struct Fish;

impl Shell for Fish {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        clap_complete::Shell::Fish
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
            return Ok(format!("set -gx PATH {path:?} $PATH;"));
        }

        let storage = multishell_storage
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't convert multishell storage path to string"))?;
        let storage = super::windows_compat::maybe_fix_windows_path(storage)
            .unwrap_or_else(|| storage.to_string());
        let expected_parts = if cfg!(windows) { 1 } else { 2 };
        let expected_last = if cfg!(windows) { "" } else { "bin" };

        Ok(format!(
            r#"begin
    set -l __fnm_storage {storage}
    set -l __fnm_prefix "$__fnm_storage/"
    set -l __fnm_prefix_length (string length -- "$__fnm_prefix")
    set -l __fnm_new_path
    for __fnm_entry in $PATH
        set -l __fnm_is_stale 0
        if test (string sub -s 1 -l $__fnm_prefix_length -- "$__fnm_entry") = "$__fnm_prefix"
            set -l __fnm_relative (string sub -s (math $__fnm_prefix_length + 1) -- "$__fnm_entry")
            set -l __fnm_parts (string split / -- "$__fnm_relative")
            if test (count $__fnm_parts) -eq {expected_parts}; and test -n "$__fnm_parts[1]"{last_check}
                set __fnm_is_stale 1
            end
        end
        if test $__fnm_is_stale -eq 0
            set -a __fnm_new_path "$__fnm_entry"
        end
    end
    set -gx PATH $__fnm_new_path {path}
end"#,
            storage = quote_fish(&storage),
            path = quote_fish(&path),
            last_check = if expected_last.is_empty() {
                String::new()
            } else {
                format!(
                    "; and test \"$__fnm_parts[2]\" = {0}",
                    quote_fish(expected_last)
                )
            },
        ))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("set -gx {name} {value:?};")
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        let version_file_exists_condition = if config.resolve_engines() {
            "test -f .node-version -o -f .nvmrc -o -f package.json"
        } else {
            "test -f .node-version -o -f .nvmrc"
        };
        let autoload_hook = match config.version_file_strategy() {
            VersionFileStrategy::Local => formatdoc!(
                r"
                    if {version_file_exists_condition}
                        fnm use --silent-if-unchanged
                    end
                ",
                version_file_exists_condition = version_file_exists_condition,
            ),
            VersionFileStrategy::Recursive => String::from(r"fnm use --silent-if-unchanged"),
        };
        Ok(formatdoc!(
            r"
                function _fnm_autoload_hook --on-variable PWD --description 'Change Node version on directory change'
                    status --is-command-substitution; and return
                    {autoload_hook}
                end
            ",
            autoload_hook = autoload_hook
        ))
    }
}
