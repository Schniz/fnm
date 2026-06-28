use crate::version_file_strategy::VersionFileStrategy;

use super::Shell;
use indoc::formatdoc;
use std::path::Path;

#[derive(Debug)]
pub struct PowerShell;

impl Shell for PowerShell {
    fn path(&self, path: &Path) -> anyhow::Result<String> {
        let path = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't read PATH"))?;
        Ok(format!(r#"$env:PATH = "{path};$env:PATH""#))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!(r#"$env:{name} = "{value}""#)
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        let version_file_exists_condition = if config.resolve_engines() {
            "(Test-Path .nvmrc) -Or (Test-Path .node-version) -Or (Test-Path package.json)"
        } else {
            "(Test-Path .nvmrc) -Or (Test-Path .node-version)"
        };
        let autoload_hook = match config.version_file_strategy() {
            VersionFileStrategy::Local => formatdoc!(
                r"
                    If ({version_file_exists_condition}) {{ & fnm use --silent-if-unchanged }}
                ",
                version_file_exists_condition = version_file_exists_condition,
            ),
            VersionFileStrategy::Recursive => String::from(r"fnm use --silent-if-unchanged"),
        };
        Ok(formatdoc!(
            r"
                function global:Set-FnmOnLoad {{ {autoload_hook} }}
                function global:Set-LocationWithFnm {{ param($path); if ($path -eq $null) {{Set-Location}} else {{Set-Location $path}}; Set-FnmOnLoad }}
                Set-Alias -Scope global cd_with_fnm Set-LocationWithFnm
                Set-Alias -Option AllScope -Scope global cd Set-LocationWithFnm
            ",
            autoload_hook = autoload_hook
        ))
    }
    fn to_clap_shell(&self) -> clap_complete::Shell {
        clap_complete::Shell::PowerShell
    }
}
