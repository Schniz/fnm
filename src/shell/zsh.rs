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

    fn use_on_cd(
        &self,
        config: &crate::config::FnmConfig,
        install_if_missing: bool,
    ) -> anyhow::Result<String> {
        let version_file_exists_condition = if config.resolve_engines() {
            "-f .node-version || -f .nvmrc || -f package.json"
        } else {
            "-f .node-version || -f .nvmrc"
        };
        let fnm_use_cmd = if install_if_missing {
            "fnm use --silent-if-unchanged --install-if-missing"
        } else {
            "fnm use --silent-if-unchanged"
        };
        let autoload_hook = match config.version_file_strategy() {
            VersionFileStrategy::Local => formatdoc!(
                r"
                    if [[ {version_file_exists_condition} ]]; then
                        {fnm_use_cmd}
                    fi
                ",
                version_file_exists_condition = version_file_exists_condition,
                fnm_use_cmd = fnm_use_cmd,
            ),
            VersionFileStrategy::Recursive => String::from(fnm_use_cmd),
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
        let output = Zsh
            .use_on_cd(&crate::config::FnmConfig::default(), false)
            .unwrap();
        assert!(output.contains("add-zsh-hook -D chpwd _fnm_autoload_hook"));
    }

}
