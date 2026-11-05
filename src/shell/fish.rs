use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use indoc::formatdoc;
use std::path::Path;

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
        let path =
            super::windows_compat::maybe_fix_windows_path(path).unwrap_or_else(|| path.to_string());
        Ok(format!("set -gx PATH {path:?} $PATH;"))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("set -gx {name} {value:?};")
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        let executable = super::shell::hook_executable()?;
        let executable = executable
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't convert executable path to string"))?;
        let executable = super::windows_compat::maybe_fix_windows_path(executable)
            .unwrap_or_else(|| executable.to_owned());
        let executable = format!(
            "'{}'",
            executable.replace('\\', "\\\\").replace('\'', "\\'")
        );
        let use_command = format!("if type -q fnm; fnm use --silent-if-unchanged; else; {executable} use --silent-if-unchanged; end");
        let version_file_exists_condition = if config.resolve_engines() {
            "test -f .node-version -o -f .nvmrc -o -f package.json"
        } else {
            "test -f .node-version -o -f .nvmrc"
        };
        let autoload_hook = match config.version_file_strategy() {
            VersionFileStrategy::Local => formatdoc!(
                r"
                    if {version_file_exists_condition}
                        {use_command}
                    end
                ",
                version_file_exists_condition = version_file_exists_condition,
            ),
            VersionFileStrategy::Recursive => use_command.clone(),
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
