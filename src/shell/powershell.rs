use std::ffi::OsString;
use crate::version_file_strategy::VersionFileStrategy;

use super::Shell;
use crate::shell::PathFormatter;
use anyhow::Result;
use indoc::{formatdoc, indoc};
use std::path::Path;
use crate::unixpath::to_unix_path;
use crate::wsl::is_wsl;

#[derive(Debug)]
pub struct PowerShell;

impl Shell for PowerShell {
    fn path(&self, path: &Path, _formatter: &PathFormatter) -> Result<String> {
        let current_path =
            std::env::var_os("PATH").ok_or_else(|| anyhow::anyhow!("Can't read PATH env var"))?;
        let mut split_paths: Vec<_> = std::env::split_paths(&current_path).collect();
        split_paths.insert(0, path.to_path_buf());

        let new_path = if is_wsl() {
            split_paths
                .iter()
                .map(|path| to_unix_path(path, &PathFormatter::Wsl))
                .collect::<Result<Vec<_>, _>>()
                .map(|vec| vec.join(":"))
                .map(OsString::from)
        } else {
            std::env::join_paths(split_paths)
                .map_err(|err| anyhow::anyhow!("Error joining paths: {}", err))
        }.map_err(|err| anyhow::anyhow!("Can't join paths: {}", err))?;

        let new_path = new_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't read PATH"))?;
        Ok(self.set_env_var("PATH", new_path))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!(r#"$env:{} = "{}""#, name, value)
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> Result<String> {
        let autoload_hook = match config.version_file_strategy() {
            VersionFileStrategy::Local => indoc!(
                r#"
                    If ((Test-Path .nvmrc) -Or (Test-Path .node-version)) { & fnm use --silent-if-unchanged }
                "#
            ),
            VersionFileStrategy::Recursive => r#"fnm use --silent-if-unchanged"#,
        };
        Ok(formatdoc!(
            r#"
                function global:Set-FnmOnLoad {{ {autoload_hook} }}
                function global:Set-LocationWithFnm {{ param($path); if ($path -eq $null) {{Set-Location}} else {{Set-Location $path}}; Set-FnmOnLoad }}
                Set-Alias -Scope global cd_with_fnm Set-LocationWithFnm
                Set-Alias -Option AllScope -Scope global cd Set-LocationWithFnm
                Set-FnmOnLoad
            "#,
            autoload_hook = autoload_hook
        ))
    }
    fn to_clap_shell(&self) -> clap_complete::Shell {
        clap_complete::Shell::PowerShell
    }

    fn format_path(&self, path: &Path, _formatter: &PathFormatter) -> Result<String> {
        if is_wsl() {
            to_unix_path(path, &PathFormatter::Wsl)
        } else {
            Ok(path.to_str().unwrap().to_string())
        }
    }
}
