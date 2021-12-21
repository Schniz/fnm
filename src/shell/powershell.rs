use crate::version_file_strategy::VersionFileStrategy;

use super::Shell;
use indoc::{formatdoc, indoc};
use std::path::Path;

#[derive(Debug)]
pub struct PowerShell;

impl Shell for PowerShell {
    fn path(&self, path: &Path) -> String {
        let current_path = std::env::var_os("PATH").expect("Can't read PATH env var");
        let mut split_paths: Vec<_> = std::env::split_paths(&current_path).collect();
        split_paths.insert(0, path.to_path_buf());
        let new_path = std::env::join_paths(split_paths).expect("Can't join paths");
        self.set_env_var("PATH", new_path.to_str().expect("Can't read PATH"))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!(r#"$env:{} = "{}""#, name, value)
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> String {
        let autoload_hook = match config.version_file_strategy() {
            VersionFileStrategy::Local => indoc!(
                r#"
                    If ((Test-Path .nvmrc) -Or (Test-Path .node-version)) { & fnm use --silent-if-unchanged }
                "#
            ),
            VersionFileStrategy::Recursive => r#"fnm use --silent-if-unchanged"#,
        };
        formatdoc!(
            r#"
                function Set-FnmOnLoad {{ {autoload_hook} }}
                function Set-LocationWithFnm {{ param($path); Set-Location $path; Set-FnmOnLoad }}
                Set-Alias cd_with_fnm Set-LocationWithFnm -Force
                Remove-Item alias:\cd
                New-Alias cd Set-LocationWithFnm
                Set-FnmOnLoad
            "#,
            autoload_hook = autoload_hook
        )
    }
    fn to_structopt_shell(&self) -> clap::Shell {
        clap::Shell::PowerShell
    }
}
