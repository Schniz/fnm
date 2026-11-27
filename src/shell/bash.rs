use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use indoc::formatdoc;
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
        let path =
            super::windows_compat::maybe_fix_windows_path(path).unwrap_or_else(|| path.to_string());
        Ok(format!("export PATH={path:?}:\"$PATH\""))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {name}={value:?}")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_on_cd_without_install_if_missing() {
        let output = Bash
            .use_on_cd(&crate::config::FnmConfig::default(), false)
            .unwrap();
        assert!(output.contains("fnm use --silent-if-unchanged"));
        assert!(!output.contains("--install-if-missing"));
    }

    #[test]
    fn use_on_cd_with_install_if_missing() {
        let output = Bash
            .use_on_cd(&crate::config::FnmConfig::default(), true)
            .unwrap();
        assert!(output.contains("fnm use --silent-if-unchanged --install-if-missing"));
    }
}
