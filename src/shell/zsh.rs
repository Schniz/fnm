use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use indoc::formatdoc;
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
        let path =
            super::windows_compat::maybe_fix_windows_path(path).unwrap_or_else(|| path.to_string());
        Ok(format!("export PATH={path:?}:$PATH"))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {name}={value:?}")
    }

    fn rehash(&self) -> Option<&'static str> {
        Some("rehash")
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        let executable = super::shell::hook_executable()?;
        let executable = executable
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't convert executable path to string"))?;
        let executable = super::windows_compat::maybe_fix_windows_path(executable)
            .unwrap_or_else(|| executable.to_owned());
        let executable = format!("'{}'", executable.replace('\'', "'\\''"));
        let use_command = format!("if command -v fnm >/dev/null 2>&1; then fnm use --silent-if-unchanged; else {executable} use --silent-if-unchanged; fi");
        let version_file_exists_condition = if config.resolve_engines() {
            "-f .node-version || -f .nvmrc || -f package.json"
        } else {
            "-f .node-version || -f .nvmrc"
        };
        let autoload_hook = match config.version_file_strategy() {
            VersionFileStrategy::Local => formatdoc!(
                r"
                    if [[ {version_file_exists_condition} ]]; then
                        {use_command}
                    fi
                ",
                version_file_exists_condition = version_file_exists_condition,
            ),
            VersionFileStrategy::Recursive => use_command.clone(),
        };
        Ok(formatdoc!(
            r"
                autoload -U add-zsh-hook
                _fnm_autoload_hook () {{
                    {autoload_hook}
                }}

                add-zsh-hook -D chpwd _fnm_autoload_hook
                add-zsh-hook chpwd _fnm_autoload_hook
            ",
            autoload_hook = autoload_hook
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_on_cd_removes_existing_hook_before_adding() {
        let output = Zsh.use_on_cd(&crate::config::FnmConfig::default()).unwrap();
        assert!(output.contains("add-zsh-hook -D chpwd _fnm_autoload_hook"));
    }
}
